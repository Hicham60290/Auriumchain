pub mod block;
pub mod chain;
pub mod genesis;
pub mod quantum_transaction;  // ← TRIPLE POST-QUANTUM TRANSACTIONS

pub use block::{Block, Transaction, TxInput, TxOutput};
pub use chain::Blockchain;
pub use genesis::{create_genesis_block, calculate_block_reward};
pub use quantum_transaction::{QuantumTransaction, CoinbaseTransaction};
pub mod utxo;
