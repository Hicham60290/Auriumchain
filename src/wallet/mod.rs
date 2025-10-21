pub mod address;
pub mod keys;
pub mod quantum_resistant;
pub mod secure_wallet;
pub mod quantum_max;  // ← TRIPLE POST-QUANTUM SECURITY

pub use quantum_resistant::{AddressType, QuantumProtection, AddressGenerator};
pub use secure_wallet::SecureWallet;
pub use quantum_max::{MaxSecurityWallet, TripleSignature, TriplePublicKeys};
