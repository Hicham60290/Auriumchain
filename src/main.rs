mod blockchain;
mod network;
mod mining;
mod wallet;
mod storage;
mod rpc;
mod utils;

use blockchain::Blockchain;
use mining::start_mining;
use rpc::start_rpc_server;
use clap::Parser;
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(name = "AuriumChain")]
#[command(about = "AuriumChain - Fast & Eco-Efficient Blockchain", long_about = None)]
struct Args {
    /// Port P2P du nœud
    #[arg(short, long, default_value = "3001")]
    port: u16,

    /// Port RPC
    #[arg(short, long, default_value = "8001")]
    rpc_port: u16,

    /// Est-ce le nœud Genesis?
    #[arg(short, long, default_value = "false")]
    genesis: bool,

    /// Activer le minage
    #[arg(short, long, default_value = "false")]
    mining: bool,
    
    /// Adresse du wallet mineur
    #[arg(short, long, default_value = "")]
    wallet: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    let args = Args::parse();

    println!("╔═══════════════════════════════════════╗");
    println!("║       🌟 AURIUMCHAIN NODE 🌟          ║");
    println!("╚═══════════════════════════════════════╝");
    println!("Port P2P:  {}", args.port);
    println!("Port RPC:  {}", args.rpc_port);
    println!("Genesis:   {}", args.genesis);
    println!("Mining:    {}", args.mining);
    println!();

    // Créer la blockchain
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));
    
    println!("✅ Blockchain initialized!");
    println!("   Genesis hash: {}", blockchain.lock().unwrap().get_latest_block().hash);
    println!();

    // Démarrer le serveur RPC
    let blockchain_rpc = blockchain.clone();
    tokio::spawn(async move {
        start_rpc_server(args.rpc_port, blockchain_rpc).await;
    });

    // Démarrer le minage si activé
    if args.mining {
        let blockchain_mining = blockchain.clone();
        let wallet_addr = if args.wallet.is_empty() {
            format!("MINER_NODE_{}", args.port)
        } else {
            args.wallet
        };

        println!("⛏️  Mining enabled for wallet: {}", wallet_addr);
        
        tokio::spawn(async move {
            start_mining(blockchain_mining, wallet_addr).await;
        });
    }

    println!("🌐 Node is running!");
    println!("📡 RPC API: http://localhost:{}/status", args.rpc_port);
    println!("\n💡 Press Ctrl+C to stop\n");
    
    // Garder le programme actif
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n👋 Shutting down gracefully...");
}
