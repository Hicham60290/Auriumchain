pub mod blockchain {
    pub mod block;
    pub mod chain;
    pub mod genesis;

    pub use block::{Block, Transaction, TxInput, TxOutput};
    pub use chain::Blockchain;
    pub use genesis::{calculate_block_reward, create_genesis_block};
}

pub mod security {
    pub mod monitor;
    pub mod protection;
    pub mod validator;

    pub use monitor::SecurityMonitor;
    pub use protection::NetworkProtection;
    pub use validator::SecurityValidator;
}

pub mod mining;
pub mod network;
pub mod p2p;
pub mod rpc;
pub mod storage;
pub mod utils;
pub mod wallet;
