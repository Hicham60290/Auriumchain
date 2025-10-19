pub mod block;
pub mod chain;
pub mod genesis;

pub use block::{Block, Transaction, TxInput, TxOutput};
pub use chain::Blockchain;
pub use genesis::{create_genesis_block, calculate_block_reward};
