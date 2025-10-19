use crate::blockchain::{Block, Transaction};
use anyhow::{Result, anyhow};
use chrono::Utc;
use std::collections::HashSet;

pub struct SecurityValidator {
    pub max_block_size: usize,
    pub max_transactions_per_block: usize,
    pub max_future_timestamp: i64,
}

impl SecurityValidator {
    pub fn new() -> Self {
        SecurityValidator {
            max_block_size: 4_000_000,
            max_transactions_per_block: 10_000,
            max_future_timestamp: 7200,
        }
    }

    pub fn validate_block_strict(&self, block: &Block, previous: &Block) -> Result<()> {
        self.validate_timestamp(block)?;
        self.validate_size(block)?;
        self.validate_transactions(block)?;
        self.validate_reward(block)?;
        self.validate_pow(block)?;
        self.validate_chain_link(block, previous)?;
        self.detect_double_spend(block)?;
        Ok(())
    }

    fn validate_timestamp(&self, block: &Block) -> Result<()> {
        let now = Utc::now().timestamp();
        
        if block.timestamp > now + self.max_future_timestamp {
            return Err(anyhow!("Block timestamp too far in future"));
        }
        
        if block.timestamp < 0 {
            return Err(anyhow!("Block timestamp is negative"));
        }
        
        Ok(())
    }

    fn validate_size(&self, block: &Block) -> Result<()> {
        let block_size = bincode::serialize(block)
            .map_err(|e| anyhow!("Failed to serialize block: {}", e))?
            .len();
        
        if block_size > self.max_block_size {
            return Err(anyhow!("Block size exceeds maximum"));
        }
        
        if block.transactions.len() > self.max_transactions_per_block {
            return Err(anyhow!("Too many transactions"));
        }
        
        Ok(())
    }

    fn validate_transactions(&self, block: &Block) -> Result<()> {
        if block.transactions.is_empty() {
            return Err(anyhow!("Block has no transactions"));
        }
        
        if !block.transactions[0].is_coinbase() {
            return Err(anyhow!("First transaction must be coinbase"));
        }
        
        for tx in &block.transactions[1..] {
            if tx.is_coinbase() {
                return Err(anyhow!("Non-first transaction cannot be coinbase"));
            }
        }
        
        Ok(())
    }

    fn validate_reward(&self, block: &Block) -> Result<()> {
        use crate::blockchain::genesis::calculate_block_reward;
        
        let expected_reward = calculate_block_reward(block.index);
        let coinbase = &block.transactions[0];
        let actual_reward: u64 = coinbase.outputs.iter().map(|o| o.value).sum();
        
        if actual_reward > expected_reward {
            return Err(anyhow!("Excessive mining reward"));
        }
        
        Ok(())
    }

    fn validate_pow(&self, block: &Block) -> Result<()> {
        let calculated_hash = block.calculate_hash();
        
        if calculated_hash != block.hash {
            return Err(anyhow!("Block hash mismatch"));
        }
        
        let target = "0".repeat(block.difficulty as usize);
        if !block.hash.starts_with(&target) {
            return Err(anyhow!("Block hash does not meet difficulty"));
        }
        
        Ok(())
    }

    fn validate_chain_link(&self, block: &Block, previous: &Block) -> Result<()> {
        if block.index != previous.index + 1 {
            return Err(anyhow!("Invalid block index"));
        }
        
        if block.previous_hash != previous.hash {
            return Err(anyhow!("Invalid previous hash"));
        }
        
        if block.timestamp < previous.timestamp {
            return Err(anyhow!("Block timestamp before previous"));
        }
        
        Ok(())
    }

    fn detect_double_spend(&self, block: &Block) -> Result<()> {
        let mut inputs_seen = HashSet::new();
        
        for tx in &block.transactions {
            for input in &tx.inputs {
                let key = format!("{}:{}", input.prev_tx_id, input.output_index);
                
                if inputs_seen.contains(&key) {
                    return Err(anyhow!("Double spend detected"));
                }
                
                inputs_seen.insert(key);
            }
        }
        
        Ok(())
    }
}
