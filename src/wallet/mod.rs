pub mod address;
pub mod keys;
pub mod quantum_resistant;
pub mod secure_wallet; // ← NOUVEAU

pub use quantum_resistant::{AddressGenerator, AddressType, QuantumProtection};
pub use secure_wallet::SecureWallet; // ← NOUVEAU
