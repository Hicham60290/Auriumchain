use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub miner_address: String,
    pub merkle_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub prev_tx_id: String,
    pub output_index: usize,
    pub signature: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub value: u64,
    pub address: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        difficulty: u32,
        miner_address: String,
    ) -> Self {
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
            miner_address,
            merkle_root,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.merkle_root,
            self.previous_hash,
            self.nonce,
            self.miner_address
        );
        
        let hash1 = Sha256::digest(data.as_bytes());
        let hash2 = Sha256::digest(&hash1);
        hex::encode(hash2)
    }

    pub fn mine(&mut self) {
        let target = "0".repeat(self.difficulty as usize);
        
        println!("⛏️  Mining block {} (difficulty {})...", self.index, self.difficulty);
        let start = Utc::now().timestamp();
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                let duration = Utc::now().timestamp() - start;
                println!("✅ Block {} mined in {}s!", self.index, duration);
                println!("   Hash: {}", self.hash);
                println!("   Nonce: {}", self.nonce);
                break;
            }
            
            self.nonce += 1;
            
            if self.nonce % 100_000 == 0 {
                print!("   Nonce: {}...\r", self.nonce);
            }
        }
    }

    pub fn is_valid(&self, previous_block: &Block) -> bool {
        if self.index != previous_block.index + 1 {
            return false;
        }

        if self.previous_hash != previous_block.hash {
            return false;
        }

        if self.hash != self.calculate_hash() {
            return false;
        }

        let target = "0".repeat(self.difficulty as usize);
        if !self.hash.starts_with(&target) {
            return false;
        }

        true
    }

    pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }

        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let data = serde_json::to_string(tx).unwrap();
                let hash = Sha256::digest(data.as_bytes());
                hex::encode(hash)
            })
            .collect();

        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();

            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0])
                };

                let hash = Sha256::digest(combined.as_bytes());
                new_hashes.push(hex::encode(hash));
            }

            hashes = new_hashes;
        }

        hashes[0].clone()
    }
}

impl Transaction {
    pub fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        let timestamp = Utc::now().timestamp();
        let id = Self::calculate_id(&inputs, &outputs, timestamp);

        Transaction {
            id,
            inputs,
            outputs,
            timestamp,
            signature: String::new(),
        }
    }

    fn calculate_id(inputs: &[TxInput], outputs: &[TxOutput], timestamp: i64) -> String {
        let data = format!("{:?}{:?}{}", inputs, outputs, timestamp);
        let hash = Sha256::digest(data.as_bytes());
        hex::encode(hash)
    }

    pub fn total_input(&self) -> u64 {
        0
    }

    pub fn total_output(&self) -> u64 {
        self.outputs.iter().map(|o| o.value).sum()
    }
    
    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}
