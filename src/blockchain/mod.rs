pub mod block;
pub mod chain;
pub mod transaction;
pub mod genesis;

// Re-export des types principaux
pub use block::{Block, Transaction, TxInput, TxOutput};
pub use chain::Blockchain;
pub use genesis::{create_genesis_block, calculate_block_reward};
pub mod block;
pub mod chain;
pub mod genesis;
pub mod fees;  // ← NOUVEAU

pub use block::{Block, Transaction, TxInput, TxOutput};
pub use chain::Blockchain;
pub use genesis::{create_genesis_block, calculate_block_reward};
pub use fees::{TransactionFees, Priority, FeeEstimate};  // ← NOUVEAU
