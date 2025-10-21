use rocksdb::{DB, Options, WriteBatch, IteratorMode};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::blockchain::{Block, Blockchain};

/// RocksDB storage implementation for AuriumChain
pub struct BlockchainDB {
    db: DB,
}

impl BlockchainDB {
    /// Open or create a new RocksDB database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, rocksdb::Error> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
        opts.increase_parallelism(num_cpus::get() as i32);
        opts.set_max_open_files(10000);
        opts.set_keep_log_file_num(10);
        opts.set_max_manifest_file_size(1024 * 1024 * 100); // 100 MB

        let db = DB::open(&opts, path)?;
        Ok(BlockchainDB { db })
    }

    /// Save a block to the database
    pub fn save_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("block:{}", block.index);
        let value = bincode::serialize(block)?;
        self.db.put(key.as_bytes(), value)?;

        // Update latest block index
        self.db.put(b"latest_index", block.index.to_string().as_bytes())?;

        // Update block hash index for quick lookups
        let hash_key = format!("hash:{}", block.hash);
        self.db.put(hash_key.as_bytes(), block.index.to_string().as_bytes())?;

        Ok(())
    }

    /// Get a block by index
    pub fn get_block(&self, index: u64) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let key = format!("block:{}", index);
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let block: Block = bincode::deserialize(&bytes)?;
                Ok(Some(block))
            },
            None => Ok(None),
        }
    }

    /// Get a block by hash
    pub fn get_block_by_hash(&self, hash: &str) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let hash_key = format!("hash:{}", hash);
        match self.db.get(hash_key.as_bytes())? {
            Some(index_bytes) => {
                let index_str = String::from_utf8(index_bytes.to_vec())?;
                let index: u64 = index_str.parse()?;
                self.get_block(index)
            },
            None => Ok(None),
        }
    }

    /// Get the latest block index
    pub fn get_latest_index(&self) -> Result<Option<u64>, Box<dyn std::error::Error>> {
        match self.db.get(b"latest_index")? {
            Some(bytes) => {
                let index_str = String::from_utf8(bytes.to_vec())?;
                Ok(Some(index_str.parse()?))
            },
            None => Ok(None),
        }
    }

    /// Get the chain height (number of blocks)
    pub fn get_chain_height(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match self.get_latest_index()? {
            Some(index) => Ok(index + 1),
            None => Ok(0),
        }
    }

    /// Load entire blockchain from database
    pub fn load_blockchain(&self) -> Result<Blockchain, Box<dyn std::error::Error>> {
        let mut blockchain = Blockchain::new();

        let latest_index = match self.get_latest_index()? {
            Some(idx) => idx,
            None => return Ok(blockchain), // Empty blockchain
        };

        // Load all blocks in order
        for i in 0..=latest_index {
            if let Some(block) = self.get_block(i)? {
                blockchain.chain.push(block);
            }
        }

        Ok(blockchain)
    }

    /// Save entire blockchain to database (batch operation)
    pub fn save_blockchain(&self, blockchain: &Blockchain) -> Result<(), Box<dyn std::error::Error>> {
        let mut batch = WriteBatch::default();

        for block in &blockchain.chain {
            let key = format!("block:{}", block.index);
            let value = bincode::serialize(block)?;
            batch.put(key.as_bytes(), value);

            // Update hash index
            let hash_key = format!("hash:{}", block.hash);
            batch.put(hash_key.as_bytes(), block.index.to_string().as_bytes());
        }

        // Update latest index
        if let Some(last_block) = blockchain.chain.last() {
            batch.put(b"latest_index", last_block.index.to_string().as_bytes());
        }

        self.db.write(batch)?;
        Ok(())
    }

    /// Get all blocks in a range
    pub fn get_blocks_range(&self, start: u64, end: u64) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
        let mut blocks = Vec::new();

        for i in start..=end {
            if let Some(block) = self.get_block(i)? {
                blocks.push(block);
            }
        }

        Ok(blocks)
    }

    /// Delete a block and all subsequent blocks (for chain reorganization)
    pub fn delete_blocks_from(&self, from_index: u64) -> Result<(), Box<dyn std::error::Error>> {
        let latest_index = match self.get_latest_index()? {
            Some(idx) => idx,
            None => return Ok(()),
        };

        let mut batch = WriteBatch::default();

        for i in from_index..=latest_index {
            if let Some(block) = self.get_block(i)? {
                let key = format!("block:{}", i);
                batch.delete(key.as_bytes());

                let hash_key = format!("hash:{}", block.hash);
                batch.delete(hash_key.as_bytes());
            }
        }

        // Update latest index
        if from_index > 0 {
            batch.put(b"latest_index", (from_index - 1).to_string().as_bytes());
        } else {
            batch.delete(b"latest_index");
        }

        self.db.write(batch)?;
        Ok(())
    }

    /// Get database statistics
    pub fn get_stats(&self) -> Result<String, Box<dyn std::error::Error>> {
        let height = self.get_chain_height()?;
        let size_estimate = self.db.property_value("rocksdb.estimate-live-data-size")?
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        Ok(format!(
            "Chain Height: {}\nEstimated DB Size: {} MB\nCompression: LZ4",
            height,
            size_estimate / 1024 / 1024
        ))
    }

    /// Compact database to optimize storage
    pub fn compact(&self) -> Result<(), rocksdb::Error> {
        self.db.compact_range(None::<&[u8]>, None::<&[u8]>);
        Ok(())
    }

    /// UTXO Management

    /// Save UTXO set
    pub fn save_utxo(&self, tx_id: &str, output_index: u32, value: u64, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let key = format!("utxo:{}:{}", tx_id, output_index);
        let utxo_data = format!("{}:{}", value, address);
        self.db.put(key.as_bytes(), utxo_data.as_bytes())?;
        Ok(())
    }

    /// Remove UTXO (when spent)
    pub fn remove_utxo(&self, tx_id: &str, output_index: u32) -> Result<(), rocksdb::Error> {
        let key = format!("utxo:{}:{}", tx_id, output_index);
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    /// Get UTXO
    pub fn get_utxo(&self, tx_id: &str, output_index: u32) -> Result<Option<(u64, String)>, Box<dyn std::error::Error>> {
        let key = format!("utxo:{}:{}", tx_id, output_index);
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let data = String::from_utf8(bytes.to_vec())?;
                let parts: Vec<&str> = data.split(':').collect();
                if parts.len() == 2 {
                    let value: u64 = parts[0].parse()?;
                    let address = parts[1].to_string();
                    Ok(Some((value, address)))
                } else {
                    Ok(None)
                }
            },
            None => Ok(None),
        }
    }

    /// Get all UTXOs for an address
    pub fn get_utxos_for_address(&self, address: &str) -> Result<Vec<(String, u32, u64)>, Box<dyn std::error::Error>> {
        let mut utxos = Vec::new();
        let prefix = b"utxo:";

        let iter = self.db.iterator(IteratorMode::From(prefix, rocksdb::Direction::Forward));

        for item in iter {
            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if !key_str.starts_with("utxo:") {
                break;
            }

            let data = String::from_utf8(value.to_vec())?;
            let parts: Vec<&str> = data.split(':').collect();

            if parts.len() == 2 && parts[1] == address {
                // Parse key: "utxo:tx_id:output_index"
                let key_parts: Vec<&str> = key_str.split(':').collect();
                if key_parts.len() == 3 {
                    let tx_id = key_parts[1].to_string();
                    let output_index: u32 = key_parts[2].parse()?;
                    let value: u64 = parts[0].parse()?;
                    utxos.push((tx_id, output_index, value));
                }
            }
        }

        Ok(utxos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_rocksdb_basic() {
        let dir = tempdir().unwrap();
        let db = BlockchainDB::open(dir.path()).unwrap();

        let block = Block::new(0, vec![], "0".to_string(), 4, "test_address".to_string());
        db.save_block(&block).unwrap();

        let retrieved = db.get_block(0).unwrap().unwrap();
        assert_eq!(retrieved.index, 0);
        assert_eq!(retrieved.previous_hash, "0");
    }

    #[test]
    fn test_utxo_management() {
        let dir = tempdir().unwrap();
        let db = BlockchainDB::open(dir.path()).unwrap();

        db.save_utxo("tx123", 0, 1000, "address1").unwrap();
        db.save_utxo("tx123", 1, 500, "address2").unwrap();

        let utxo = db.get_utxo("tx123", 0).unwrap().unwrap();
        assert_eq!(utxo.0, 1000);
        assert_eq!(utxo.1, "address1");

        let utxos = db.get_utxos_for_address("address1").unwrap();
        assert_eq!(utxos.len(), 1);
        assert_eq!(utxos[0].2, 1000);
    }
}
