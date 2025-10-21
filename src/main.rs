use tokio::sync::RwLock;
use std::sync::Arc;
use auriumchain::blockchain::Blockchain;
use auriumchain::rpc::server::start_rpc_server;
use auriumchain::p2p::{PeerManager, SyncManager, NetworkSecurity, P2PServer};
use auriumchain::storage::db::BlockchainDB;
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

    #[arg(long, help = "Use RocksDB for storage (production mode)")]
    rocksdb_path: Option<String>,

    #[arg(long, help = "Miner address to receive block rewards (AUR3...)")]
    miner_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();

    let storage_mode = if args.rocksdb_path.is_some() {
        "RocksDB (Production)"
    } else {
        "JSON (Development)"
    };

    println!("AuriumChain Node - TLS P2P Edition");
    println!("Port P2P:  {}", args.port);
    println!("Port RPC:  {}", args.rpc_port);
    println!("Genesis:   {}", args.genesis);
    println!("Mining:    {}", args.mining);
    println!("Storage:   {}", storage_mode);

    if let Some(ref db_path) = args.rocksdb_path {
        println!("RocksDB:   {}", db_path);
    } else {
        println!("Data file: {}", args.data_file);
    }

    // Miner address - configurable via CLI or use default
    let wallet_addr = args.miner_address.clone().unwrap_or_else(|| {
        let default_addr = "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk".to_string();
        println!("⚠️  WARNING: Using default miner address: {}", default_addr);
        println!("⚠️  For production, use --miner-address to specify your own address!");
        default_addr
    });

    println!("Miner addr: {}", wallet_addr);

    // Charger ou créer la blockchain
    let (blockchain, db_handle) = if let Some(ref rocksdb_path) = args.rocksdb_path {
        // Mode RocksDB
        println!("\n🗄️  Using RocksDB storage...");
        let db = BlockchainDB::open(rocksdb_path)?;

        let blockchain = if args.genesis {
            println!("Creating new Genesis blockchain...");
            let mut chain = Blockchain::new();

            let genesis = auriumchain::blockchain::Block::new(
                0,
                vec![],
                "0".to_string(),
                4,
                wallet_addr.clone(),
            );
            chain.chain.push(genesis.clone());

            db.save_blockchain(&chain)?;
            println!("Genesis block saved to RocksDB!");
            chain
        } else {
            match db.load_blockchain() {
                Ok(chain) => {
                    println!("✓ Blockchain loaded from RocksDB: {} blocks", chain.chain.len());
                    chain
                },
                Err(e) => {
                    println!("Error loading blockchain from RocksDB: {}", e);
                    println!("Creating new blockchain...");
                    let mut chain = Blockchain::new();

                    let genesis = auriumchain::blockchain::Block::new(
                        0,
                        vec![],
                        "0".to_string(),
                        4,
                        wallet_addr.clone(),
                    );
                    chain.chain.push(genesis);
                    chain
                }
            }
        };

        (blockchain, Some(db))
    } else {
        // Mode JSON (legacy)
        println!("\n📄 Using JSON storage (development mode)...");
        let blockchain = if args.genesis {
            println!("Creating new Genesis blockchain...");
            let mut chain = Blockchain::new();

            let genesis = auriumchain::blockchain::Block::new(
                0,
                vec![],
                "0".to_string(),
                4,
                wallet_addr.clone(),
            );
            chain.chain.push(genesis.clone());

            if let Err(e) = chain.save_to_file(&args.data_file) {
                eprintln!("Error saving blockchain: {}", e);
            } else {
                println!("Blockchain saved: {} blocks to {}", chain.chain.len(), args.data_file);
            }

            println!("Genesis Block created!");
            chain
        } else {
            println!("Loading blockchain from {}...", args.data_file);
            match Blockchain::load_from_file(&args.data_file) {
                Ok(chain) => {
                    println!("Blockchain loaded: {} blocks from {}", chain.chain.len(), args.data_file);
                    println!("Loaded {} blocks", chain.chain.len());
                    chain
                },
                Err(e) => {
                    println!("Error loading blockchain: {}", e);
                    println!("Creating new blockchain...");
                    let mut chain = Blockchain::new();

                    let genesis = auriumchain::blockchain::Block::new(
                        0,
                        vec![],
                        "0".to_string(),
                        4,
                        wallet_addr.clone(),
                    );
                    chain.chain.push(genesis);
                    chain
                }
            }
        };

        (blockchain, None)
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
        }
    }
    
    // **NOUVELLE FONCTIONNALITÉ : Synchronisation automatique périodique**
    let sync_manager_periodic = sync_manager.clone();
    let peer_manager_sync = peer_manager.clone();
    tokio::spawn(async move {
        loop {
            // Attendre 30 secondes avant chaque cycle de synchronisation
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            
            let peers = peer_manager_sync.get_all_peers().await;
            if !peers.is_empty() {
                println!("Starting periodic sync with {} peers...", peers.len());
                
                for peer_addr in peers {
                    match sync_manager_periodic.sync_with_peer(peer_addr).await {
                        Ok(synced) => {
                            if synced {
                                println!("✅ Synchronized new blocks from peer: {}", peer_addr);
                            }
                        },
                        Err(e) => {
                            println!("❌ Sync failed with peer {}: {}", peer_addr, e);
                        }
                    }
                }
            }
        }
    });
    
    let blockchain_rpc = blockchain.clone();
    let blockchain_mining = blockchain.clone();
    let data_file_mining = args.data_file.clone();
    let rocksdb_mining = args.rocksdb_path.clone();
    let sync_manager_mining = sync_manager.clone();

    // Wrap db_handle in Arc for sharing
    let db_handle = db_handle.map(Arc::new);
    let db_handle_mining = db_handle.clone();

    // Démarrer RPC
    tokio::spawn(async move {
        if let Err(e) = start_rpc_server(blockchain_rpc, args.rpc_port).await {
            eprintln!("RPC error: {}", e);
        }
    });

    // **AMÉLIORATION : Mining avec propagation automatique des blocs**
    if args.mining {
        tokio::spawn(async move {
            loop {
                let start = std::time::Instant::now();

                let new_block = {
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

                        // Save using RocksDB if available, otherwise JSON
                        if let Some(ref db) = db_handle_mining {
                            if let Err(e) = db.save_block(&new_block) {
                                eprintln!("Error saving block to RocksDB: {}", e);
                            } else {
                                println!("✓ Block saved to RocksDB");
                            }
                        } else {
                            if let Err(e) = chain.save_to_file(&data_file_mining) {
                                eprintln!("Error saving blockchain: {}", e);
                            } else {
                                println!("Blockchain saved: {} blocks to {}", chain.chain.len(), data_file_mining);
                            }
                        }

                        println!("⛏️  Block {} mined and saved!", new_block.index);
                        println!("   Hash: {}", new_block.hash);
                        println!("   Chain: {} blocks", chain.chain.len());

                        Some(new_block)
                    } else {
                        None
                    }
                };

                // **NOUVELLE FONCTIONNALITÉ : Propager le nouveau bloc vers tous les peers**
                if let Some(block) = new_block {
                    tokio::spawn({
                        let sync_manager = sync_manager_mining.clone();
                        async move {
                            if let Err(e) = sync_manager.broadcast_new_block(block).await {
                                eprintln!("Failed to broadcast new block: {}", e);
                            } else {
                                println!("📡 New block broadcasted to all peers");
                            }
                        }
                    });
                }

                let elapsed = start.elapsed();
                if elapsed.as_secs() < 30 {
                    tokio::time::sleep(tokio::time::Duration::from_secs(30 - elapsed.as_secs())).await;
                }
            }
        });
    }
    
    println!("P2P Server (TLS) listening on 0.0.0.0:{}", args.port);
    println!("RPC Server listening on http://0.0.0.0:{}", args.rpc_port);
    println!("TLS P2P Node running! Press Ctrl+C to stop");
    
    // **NOUVELLE FONCTIONNALITÉ : Synchronisation initiale au démarrage**
    if !args.genesis {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        println!("🔄 Starting initial synchronization...");
        
        let peers = peer_manager.get_all_peers().await;
        for peer_addr in peers {
            match sync_manager.sync_with_peer(peer_addr).await {
                Ok(synced) => {
                    if synced {
                        println!("✅ Initial sync completed with peer: {}", peer_addr);
                    } else {
                        println!("ℹ️  Already up to date with peer: {}", peer_addr);
                    }
                },
                Err(e) => {
                    println!("❌ Initial sync failed with peer {}: {}", peer_addr, e);
                }
            }
        }
    }
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}