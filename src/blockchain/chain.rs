use super::block::{Block, Transaction, TxOutput};
use super::genesis::{create_genesis_block, calculate_block_reward};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = create_genesis_block();
        
        Blockchain {
            chain: vec![genesis],
            difficulty: 4,
            pending_transactions: vec![],
        }
    }

    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let reward = calculate_block_reward(self.chain.len() as u64);
        
        println!("\n⛏️  Mining new block...");
        println!("   Reward: {} AUR", reward as f64 / 100_000_000.0);
        
        let mut transactions = self.pending_transactions.clone();
        
        let coinbase = Transaction {
            id: format!("coinbase-{}", self.chain.len()),
            inputs: vec![],
            outputs: vec![TxOutput {
                value: reward,
                address: miner_address.clone(),
            }],
            timestamp: chrono::Utc::now().timestamp(),
            signature: String::new(),
        };
        
        transactions.insert(0, coinbase);

        let latest = self.get_latest_block();
        let mut new_block = Block::new(
            latest.index + 1,
            transactions,
            latest.hash.clone(),
            self.difficulty,
            miner_address,
        );

        new_block.mine();
        self.chain.push(new_block);
        self.pending_transactions.clear();
        
        println!("✅ Block added to chain (height: {})\n", self.chain.len() - 1);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if !current.is_valid(previous) {
                return false;
            }
        }
        true
    }

    pub fn adjust_difficulty(&mut self) {
        const ADJUSTMENT_INTERVAL: usize = 2016;
        const TARGET_BLOCK_TIME: i64 = 30;
        
        if self.chain.len() % ADJUSTMENT_INTERVAL != 0 || self.chain.len() < ADJUSTMENT_INTERVAL {
            return;
        }

        let recent_blocks = &self.chain[self.chain.len()-ADJUSTMENT_INTERVAL..];
        let time_taken = recent_blocks.last().unwrap().timestamp 
                       - recent_blocks.first().unwrap().timestamp;
        
        let expected_time = ADJUSTMENT_INTERVAL as i64 * TARGET_BLOCK_TIME;
        
        if time_taken < expected_time / 2 {
            self.difficulty += 1;
            println!("⬆️  Difficulty increased to {}", self.difficulty);
        } else if time_taken > expected_time * 2 {
            self.difficulty = self.difficulty.saturating_sub(1).max(1);
            println!("⬇️  Difficulty decreased to {}", self.difficulty);
        }
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        let mut balance = 0u64;

        for block in &self.chain {
            for tx in &block.transactions {
                for output in &tx.outputs {
                    if output.address == address {
                        balance += output.value;
                    }
                }
            }
        }

        balance
    }
}
