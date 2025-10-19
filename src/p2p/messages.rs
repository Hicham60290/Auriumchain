use serde::{Serialize, Deserialize};
use crate::blockchain::Block;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum P2PMessage {
    Handshake { 
        version: String, 
        chain_length: u64,
        peer_id: String,
        genesis_hash: String,
    },
    ChainLengthQuery,
    ChainLengthResponse { length: u64 },
    RequestBlocks { 
        from_height: u64, 
        to_height: u64 
    },
    SendBlocks { 
        blocks: Vec<Block> 
    },
    NewBlock { 
        block: Block 
    },
    Ping,
    Pong,
}

impl P2PMessage {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }
    
    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(data)?)
    }
}
