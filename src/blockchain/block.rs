use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;
use chrono::Utc;
use std::time::Instant;
use crate::utils::{quantum_block_hash, quantum_hash_hex};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TxInput {
    pub prev_tx_id: String,
    pub output_index: usize,
    pub signature: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            difficulty,
            miner_address,
            merkle_root,
        };

        // Calculate the hash immediately so all blocks have valid hashes
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        // Use quantum-resistant double hashing: SHA-256 + BLAKE3
        quantum_block_hash(
            self.index,
            self.timestamp,
            &self.previous_hash,
            &self.merkle_root,
            self.nonce,
            self.difficulty as usize,
            &self.miner_address,
        )
    }

    pub fn mine(&mut self) {
        let target = "0".repeat(self.difficulty as usize);
        let start = Instant::now();
        
        println!("⛏️  Mining block {} (difficulty {})...", self.index, self.difficulty);
        
        loop {
            self.hash = self.calculate_hash();
            
            if self.hash.starts_with(&target) {
                let duration = start.elapsed();
                println!("✅ Block {} mined in {}s!", self.index, duration.as_secs());
                println!("   Hash: {}", self.hash);
                println!("   Nonce: {}", self.nonce);
                break;
            }
            
            self.nonce += 1;
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

        // Use quantum-resistant hashing for each transaction
        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let data = serde_json::to_string(tx).unwrap();
                quantum_hash_hex(data.as_bytes())
            })
            .collect();

        // Build Merkle tree with quantum-resistant hashing
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();

            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0])
                };

                new_hashes.push(quantum_hash_hex(combined.as_bytes()));
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
        // Use quantum-resistant hashing for transaction ID
        quantum_hash_hex(data.as_bytes())
    }

    pub fn total_input(&self) -> u64 {
        self.inputs.iter().map(|_| 0).sum()
    }

    pub fn total_output(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
}