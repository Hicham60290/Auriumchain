use crate::blockchain::Blockchain;  // <-- Changez cette ligne
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use log::info;  // <-- Retirez "warn"

// ... le reste du code reste identique

pub struct Miner {
    pub address: String,
    pub mining: bool,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Miner {
            address,
            mining: false,
        }
    }

    pub async fn start(&mut self, blockchain: Arc<Mutex<Blockchain>>) {
        self.mining = true;
        info!("⛏️  Miner started for address: {}", self.address);

        loop {
            if !self.mining {
                break;
            }

            // Attendre un peu avant de miner
            sleep(Duration::from_secs(10)).await;

            {
                let mut chain = blockchain.lock().unwrap();
                
                if chain.pending_transactions.is_empty() {
                    info!("⏸️  No pending transactions, waiting...");
                    continue;
                }

                info!("⛏️  Mining new block with {} transactions", 
                      chain.pending_transactions.len());
                
                chain.mine_pending_transactions(self.address.clone());
                
                // Ajuster la difficulté tous les 2016 blocs
                chain.adjust_difficulty();
            }
        }

        info!("🛑 Miner stopped");
    }

    pub fn stop(&mut self) {
        self.mining = false;
    }
}

pub async fn start_mining(blockchain: Arc<Mutex<Blockchain>>, miner_address: String) {
    let mut miner = Miner::new(miner_address);
    miner.start(blockchain).await;
}
