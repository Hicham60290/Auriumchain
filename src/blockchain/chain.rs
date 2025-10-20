use serde::{Deserialize, Serialize};
use crate::blockchain::{Block, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: Vec::new(),
            difficulty: 4,
        }
    }

    pub fn get_latest_block(&self) -> Option<&Block> {
        self.chain.last()
    }

    pub fn add_block(&mut self, mut block: Block) {
        if let Some(latest_block) = self.get_latest_block() {
            block.previous_hash = latest_block.hash.clone();
            block.index = latest_block.index + 1;
        }
        
        block.mine();
        self.chain.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.chain.iter()
            .filter(|block| block.miner_address == address)
            .count() as u64 * 50
    }

    pub fn get_chain_length(&self) -> usize {
        self.chain.len()
    }

    pub fn get_total_supply(&self) -> u64 {
        self.chain.len() as u64 * 50
    }

    pub fn get_difficulty(&self) -> usize {
        self.difficulty
    }

    pub fn validate_new_block(&self, block: &Block) -> bool {
        if let Some(latest_block) = self.get_latest_block() {
            if block.index != latest_block.index + 1 {
                return false;
            }
            
            if block.previous_hash != latest_block.hash {
                return false;
            }
        }

        block.hash == block.calculate_hash()
    }
}

pub fn create_genesis_block() -> Block {
    use crate::blockchain::{TxInput, TxOutput};
    
    let genesis_tx = Transaction::new(
        vec![TxInput {
            prev_tx_id: "0".to_string(),
            output_index: 0,
            public_key: "genesis".to_string(),
            signature: "genesis_sig".to_string(),
        }],
        vec![TxOutput {
            address: "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk".to_string(),
            value: 50,
        }]
    );

    Block::new(
        0,
        vec![genesis_tx],
        "0".to_string(),
        4,
        "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk".to_string(),
    )
}
