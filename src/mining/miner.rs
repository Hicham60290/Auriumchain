use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;

pub struct Miner {
    pub address: String,
}

impl Miner {
    pub fn new(address: String) -> Self {
        Self { address }
    }
    
    pub fn mine_block(&self, blockchain: Arc<Mutex<Blockchain>>) {
        // Implémentation simplifiée pour éviter les erreurs
        println!("Mining for address: {}", self.address);
    }
}
