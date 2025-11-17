use crate::blockchain::{Block, Transaction};
use serde::{Deserialize, Serialize};

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

    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        // SECURITY: Prevent modification or replacement of Genesis block
        if block.index == 0 {
            return Err("Cannot modify or replace Genesis block".to_string());
        }

        // Get the latest block for validation
        let latest_block = self
            .get_latest_block()
            .ok_or_else(|| "Blockchain is empty, cannot add block".to_string())?;

        // SECURITY: Validate block index is sequential
        if block.index != latest_block.index + 1 {
            return Err(format!(
                "Invalid block index: expected {}, got {}",
                latest_block.index + 1,
                block.index
            ));
        }

        // SECURITY: Validate previous hash matches
        if block.previous_hash != latest_block.hash {
            return Err("Previous hash does not match latest block hash".to_string());
        }

        // SECURITY: Validate Proof of Work
        let target = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&target) {
            return Err(format!(
                "Invalid Proof of Work: hash does not meet difficulty requirement (expected {} leading zeros)",
                self.difficulty
            ));
        }

        // SECURITY: Validate hash is correctly calculated
        if block.hash != block.calculate_hash() {
            return Err("Block hash is invalid or tampered".to_string());
        }

        // SECURITY: Validate block reward (coinbase transaction)
        if !block.transactions.is_empty() {
            let coinbase = &block.transactions[0];
            if coinbase.inputs.is_empty() {
                // This is a coinbase transaction, validate the reward
                let total_output: u64 = coinbase.outputs.iter().map(|o| o.value).sum();
                let expected_reward =
                    crate::blockchain::genesis::calculate_block_reward(block.index as u64);

                if total_output > expected_reward {
                    return Err(format!(
                        "Excessive mining reward: got {}, expected max {}",
                        total_output, expected_reward
                    ));
                }
            }
        }

        // All validations passed, add the block
        self.chain.push(block);
        Ok(())
    }

    /// Add a block without validation (used internally for genesis and sync)
    pub fn add_block_unchecked(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn is_chain_valid(&self) -> bool {
        if self.chain.is_empty() {
            return false;
        }

        // Validate Genesis block
        let genesis = &self.chain[0];
        if genesis.index != 0 {
            return false;
        }
        if genesis.hash != genesis.calculate_hash() {
            return false;
        }

        // Validate all subsequent blocks
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
        self.chain
            .iter()
            .filter(|block| block.miner_address == address)
            .count() as u64
            * 50
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
        }],
    );

    Block::new(
        0,
        vec![genesis_tx],
        "0".to_string(),
        4,
        "AUR3ZnxihprBGetUiMoHwRWZbcyU94TzP52Jkk".to_string(),
    )
}
