pub mod blockchain {
    pub mod block;
    pub mod chain;
    pub mod genesis;
    
    pub use block::{Block, Transaction, TxInput, TxOutput};
    pub use chain::Blockchain;
    pub use genesis::{create_genesis_block, calculate_block_reward};
}

pub mod security {
    pub mod validator;
    pub mod monitor;
    pub mod protection;
    
    pub use validator::SecurityValidator;
    pub use monitor::SecurityMonitor;
    pub use protection::NetworkProtection;
}

pub mod network;
pub mod mining;
pub mod wallet;
pub mod storage;
pub mod rpc;
pub mod utils;
