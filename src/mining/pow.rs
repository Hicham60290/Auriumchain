use sha2::{Digest, Sha256};
use crate::blockchain::Block;

pub struct ProofOfWork {
    pub difficulty: u32,
    pub target_block_time: u64,
}

impl ProofOfWork {
    pub fn new(difficulty: u32) -> Self {
        ProofOfWork {
            difficulty,
            target_block_time: 30,
        }
    }

    pub fn meets_difficulty(&self, hash: &str) -> bool {
        let target = "0".repeat(self.difficulty as usize);
        hash.starts_with(&target)
    }

    pub fn estimate_hashrate(&self, blocks: &[Block]) -> f64 {
        if blocks.len() < 2 {
            return 0.0;
        }

        let recent = &blocks[blocks.len().saturating_sub(100)..];
        let time_span = recent.last().unwrap().timestamp - recent.first().unwrap().timestamp;
        
        if time_span == 0 {
            return 0.0;
        }

        let total_work: u64 = recent.iter().map(|b| 1u64 << b.difficulty).sum();
        
        (total_work as f64) / (time_span as f64)
    }
}
