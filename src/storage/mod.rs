pub mod db;

use std::fs;
use std::path::Path;
use anyhow::Result;

impl crate::blockchain::Blockchain {
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json_data = serde_json::to_string_pretty(&self.chain)?;
        fs::write(path, json_data)?;
        println!("Blockchain saved: {} blocks to {}", self.chain.len(), path);
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self> {
        if !Path::new(path).exists() {
            println!("No blockchain file found, creating new chain");
            return Ok(Self::new());
        }
        
        let json_data = fs::read_to_string(path)?;
        let blocks: Vec<crate::blockchain::Block> = serde_json::from_str(&json_data)?;
        
        let mut blockchain = Self::new();
        blockchain.chain = blocks;
        
        println!("Blockchain loaded: {} blocks from {}", blockchain.chain.len(), path);
        Ok(blockchain)
    }
}
