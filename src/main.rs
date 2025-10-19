use tokio::sync::RwLock;
use std::sync::Arc;
use auriumchain::blockchain::Blockchain;
use auriumchain::rpc::server::start_rpc_server;
use auriumchain::p2p::{PeerManager, SyncManager, NetworkSecurity, P2PServer};
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(name = "AuriumChain")]
struct Args {
    #[arg(short = 'p', long, default_value = "3001")]
    port: u16,
    
    #[arg(short = 'r', long = "rpc-port", default_value = "8001")]
    rpc_port: u16,
    
    #[arg(short = 'g', long)]
    genesis: bool,
    
    #[arg(short = 'm', long)]
    mining: bool,
    
    #[arg(long)]
    peer: Option<String>,
    
    #[arg(long, default_value = "/tmp/auriumchain.json")]
    data_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let args = Args::parse();
    
    println!("AuriumChain Node - TLS P2P Edition");
    println!("Port P2P:  {}", args.port);
    println!("Port RPC:  {}", args.rpc_port);
    println!("Genesis:   {}", args.genesis);
    println!("Mining:    {}", args.mining);
    println!("Data file: {}", args.data_file);
    
    let wallet_addr = "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk".to_string();
    
    // Charger ou créer la blockchain
    let blockchain = if args.genesis {
        println!("Creating new Genesis blockchain...");
        let mut chain = Blockchain::new();
        let genesis = auriumchain::blockchain::Block::new(
            0, 
            vec![], 
            "0".to_string(), 
            4, 
            wallet_addr.clone()
        );
        chain.chain.push(genesis.clone());
        
        if let Err(e) = chain.save_to_file(&args.data_file) {
            eprintln!("Error saving blockchain: {}", e);
        }
        
        println!("Genesis Block created: {}", genesis.hash);
        chain
    } else {
        println!("Loading blockchain from {}...", args.data_file);
        match Blockchain::load_from_file(&args.data_file) {
            Ok(chain) => {
                println!("Loaded {} blocks", chain.chain.len());
                chain
            },
            Err(e) => {
                println!("Error loading blockchain: {}", e);
                println!("Use --genesis to create a new one");
                return Ok(());
            }
        }
    };
    
    let blockchain = Arc::new(RwLock::new(blockchain));
    let peer_manager = Arc::new(PeerManager::new(10));
    
    // Initialiser la sécurité TLS
    let security = Arc::new(NetworkSecurity::new()?);
    println!("TLS security initialized");
    
    let sync_manager = Arc::new(SyncManager::new(blockchain.clone(), peer_manager.clone()));
    
    // Démarrer serveur P2P TLS
    let p2p_server = P2PServer::new(
        blockchain.clone(),
        peer_manager.clone(),
        security.clone(),
        SocketAddr::from(([0, 0, 0, 0], args.port))
    );
    
    tokio::spawn(async move {
        if let Err(e) = p2p_server.start().await {
            eprintln!("P2P TLS server error: {}", e);
        }
    });
    
    // Ajouter peer si spécifié
    if let Some(peer_addr) = args.peer {
        if let Ok(addr) = peer_addr.parse() {
            peer_manager.add_peer(addr).await;
            println!("Added peer: {}", addr);
            
            if let Err(e) = sync_manager.sync_with_peer(addr).await {
                eprintln!("Sync error: {}", e);
            }
        }
    }
    
    let blockchain_rpc = blockchain.clone();
    let blockchain_mining = blockchain.clone();
    let data_file_mining = args.data_file.clone();
    
    // Démarrer RPC
    tokio::spawn(async move {
        if let Err(e) = start_rpc_server(blockchain_rpc, args.rpc_port).await {
            eprintln!("RPC error: {}", e);
        }
    });
    
    // Démarrer mining avec sauvegarde
    if args.mining {
        tokio::spawn(async move {
            loop {
                let start = std::time::Instant::now();
                
                {
                    let mut chain = blockchain_mining.write().await;
                    if let Some(prev_block) = chain.chain.last().cloned() {
                        let new_block = auriumchain::blockchain::Block::new(
                            prev_block.index + 1,
                            vec![],
                            prev_block.hash.clone(),
                            4,
                            wallet_addr.clone(),
                        );
                        
                        chain.chain.push(new_block.clone());
                        
                        if let Err(e) = chain.save_to_file(&data_file_mining) {
                            eprintln!("Error saving blockchain: {}", e);
                        }
                        
                        println!("Block {} mined and saved (TLS)!", new_block.index);
                        println!("   Hash: {}", new_block.hash);
                        println!("   Chain: {} blocks", chain.chain.len());
                    }
                }
                
                let elapsed = start.elapsed();
                if elapsed.as_secs() < 30 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(30 - elapsed.as_secs())).await;
                }
            }
        });
    }
    
    println!("TLS P2P Node running! Press Ctrl+C to stop");
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
