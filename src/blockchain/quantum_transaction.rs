/// Quantum-Secured Transaction with Triple Signature
///
/// This module implements transactions that use triple post-quantum signatures:
/// - ECDSA (classical security)
/// - Dilithium5 (lattice-based post-quantum)
/// - SPHINCS+ (hash-based post-quantum)

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::wallet::quantum_max::{MaxSecurityWallet, TripleSignature, TriplePublicKeys};
use crate::utils::quantum_hash_hex;

/// Transaction with maximum post-quantum security
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuantumTransaction {
    /// Sender address (AURQ format)
    pub from: String,

    /// Recipient address (any AUR* format)
    pub to: String,

    /// Amount in satoshis (1 AUR = 100,000,000 satoshis)
    pub amount: u64,

    /// Transaction timestamp (Unix epoch)
    pub timestamp: u64,

    /// Nonce to prevent replay attacks
    pub nonce: u64,

    /// Transaction fee in satoshis
    pub fee: u64,

    /// Triple signature (ECDSA + Dilithium + SPHINCS+)
    pub signature: Option<TripleSignature>,

    /// Transaction ID (quantum-resistant hash)
    pub id: String,
}

impl QuantumTransaction {
    /// Create a new unsigned quantum transaction
    pub fn new(from: String, to: String, amount: u64, nonce: u64, fee: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tx = Self {
            from,
            to,
            amount,
            timestamp,
            nonce,
            fee,
            signature: None,
            id: String::new(),
        };

        // Calculate transaction ID
        tx.id = tx.calculate_id();

        tx
    }

    /// Calculate transaction ID using quantum-resistant hashing
    fn calculate_id(&self) -> String {
        let data = format!(
            "{}{}{}{}{}{}",
            self.from, self.to, self.amount,
            self.timestamp, self.nonce, self.fee
        );
        quantum_hash_hex(data.as_bytes())
    }

    /// Get transaction hash for signing (SHA-256)
    pub fn get_signing_hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(self.from.as_bytes());
        hasher.update(self.to.as_bytes());
        hasher.update(&self.amount.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.fee.to_le_bytes());
        hasher.finalize().to_vec()
    }

    /// Sign transaction with triple security wallet
    ///
    /// This takes ~600ms due to SPHINCS+ signing
    pub fn sign(&mut self, wallet: &MaxSecurityWallet) -> Result<(), String> {
        // Verify sender address matches wallet
        if self.from != wallet.address() {
            return Err(format!(
                "Wallet address mismatch: tx from {} but wallet is {}",
                self.from, wallet.address()
            ));
        }

        println!("📝 Signing transaction {} -> {} ({} AUR)",
            &self.from[..12], &self.to[..12], self.amount as f64 / 100_000_000.0);

        let tx_hash = self.get_signing_hash();
        let signature = wallet.sign_triple(&tx_hash);

        self.signature = Some(signature);

        println!("✅ Transaction signed with triple security");

        Ok(())
    }

    /// Verify transaction signature
    ///
    /// ALL 3 signatures (ECDSA + Dilithium + SPHINCS+) must be valid
    pub fn verify_signature(&self, public_keys: &TriplePublicKeys) -> Result<bool, String> {
        if let Some(ref sig) = self.signature {
            let tx_hash = self.get_signing_hash();

            MaxSecurityWallet::verify_triple(sig, &tx_hash, public_keys)
        } else {
            Ok(false) // No signature
        }
    }

    /// Check if transaction is valid (basic checks)
    pub fn is_valid(&self) -> bool {
        // Check amount
        if self.amount == 0 {
            println!("❌ Transaction invalid: amount is zero");
            return false;
        }

        // Check addresses
        if self.from.is_empty() || self.to.is_empty() {
            println!("❌ Transaction invalid: empty address");
            return false;
        }

        // Check timestamp (not in future)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if self.timestamp > now + 300 {  // Allow 5 min clock skew
            println!("❌ Transaction invalid: timestamp in future");
            return false;
        }

        // Check signature exists
        if self.signature.is_none() {
            println!("❌ Transaction invalid: not signed");
            return false;
        }

        true
    }

    /// Get total cost (amount + fee)
    pub fn total_cost(&self) -> u64 {
        self.amount.saturating_add(self.fee)
    }

    /// Get signature size in bytes (if signed)
    pub fn signature_size(&self) -> usize {
        if let Some(ref sig) = self.signature {
            sig.size()
        } else {
            0
        }
    }

    /// Display transaction info
    pub fn display(&self) {
        println!("╔═══════════════════════════════════════════════════╗");
        println!("║         QUANTUM-SECURED TRANSACTION               ║");
        println!("╠═══════════════════════════════════════════════════╣");
        println!("║ ID:        {}...", &self.id[..16]);
        println!("║ From:      {}...", &self.from[..20]);
        println!("║ To:        {}...", &self.to[..20]);
        println!("║ Amount:    {} AUR", self.amount as f64 / 100_000_000.0);
        println!("║ Fee:       {} AUR", self.fee as f64 / 100_000_000.0);
        println!("║ Nonce:     {}", self.nonce);
        println!("║ Signed:    {}", if self.signature.is_some() { "✅ Yes (Triple)" } else { "❌ No" });
        if self.signature.is_some() {
            println!("║ Sig Size:  {} KB", self.signature_size() / 1024);
        }
        println!("╚═══════════════════════════════════════════════════╝");
    }
}

/// Coinbase transaction (mining reward) - doesn't require triple signature
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoinbaseTransaction {
    /// Miner address (recipient)
    pub to: String,

    /// Block reward in satoshis
    pub amount: u64,

    /// Transaction fees collected
    pub fees: u64,

    /// Block height
    pub block_height: u64,

    /// Timestamp
    pub timestamp: u64,

    /// Transaction ID
    pub id: String,
}

impl CoinbaseTransaction {
    /// Create a new coinbase transaction
    pub fn new(to: String, amount: u64, fees: u64, block_height: u64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tx = Self {
            to,
            amount,
            fees,
            block_height,
            timestamp,
            id: String::new(),
        };

        tx.id = tx.calculate_id();

        tx
    }

    /// Calculate transaction ID
    fn calculate_id(&self) -> String {
        let data = format!(
            "coinbase{}{}{}{}",
            self.to, self.amount, self.fees, self.block_height
        );
        quantum_hash_hex(data.as_bytes())
    }

    /// Get total reward (block reward + fees)
    pub fn total_reward(&self) -> u64 {
        self.amount.saturating_add(self.fees)
    }

    /// Display coinbase info
    pub fn display(&self) {
        println!("╔═══════════════════════════════════════════════════╗");
        println!("║         COINBASE TRANSACTION (Mining Reward)      ║");
        println!("╠═══════════════════════════════════════════════════╣");
        println!("║ Block:     #{}", self.block_height);
        println!("║ Miner:     {}...", &self.to[..20]);
        println!("║ Reward:    {} AUR", self.amount as f64 / 100_000_000.0);
        println!("║ Fees:      {} AUR", self.fees as f64 / 100_000_000.0);
        println!("║ Total:     {} AUR", self.total_reward() as f64 / 100_000_000.0);
        println!("╚═══════════════════════════════════════════════════╝");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = QuantumTransaction::new(
            "AURQ1234".to_string(),
            "AUR5678".to_string(),
            100_000_000, // 1 AUR
            0,
            1_000_000    // 0.01 AUR fee
        );

        assert_eq!(tx.from, "AURQ1234");
        assert_eq!(tx.to, "AUR5678");
        assert_eq!(tx.amount, 100_000_000);
        assert_eq!(tx.fee, 1_000_000);
        assert!(tx.signature.is_none());
    }

    #[test]
    fn test_transaction_signing_and_verification() {
        let wallet = MaxSecurityWallet::new();
        let mut tx = QuantumTransaction::new(
            wallet.address().to_string(),
            "AUR5678".to_string(),
            100_000_000,
            0,
            1_000_000
        );

        // Sign transaction
        tx.sign(&wallet).expect("Signing should succeed");

        assert!(tx.signature.is_some());

        // Verify signature
        let public_keys = wallet.export_public_keys();
        let valid = tx.verify_signature(&public_keys)
            .expect("Verification should not error");

        assert!(valid, "Signature should be valid");
    }

    #[test]
    fn test_transaction_validation() {
        let wallet = MaxSecurityWallet::new();
        let mut tx = QuantumTransaction::new(
            wallet.address().to_string(),
            "AUR5678".to_string(),
            100_000_000,
            0,
            1_000_000
        );

        // Unsigned transaction is invalid
        assert!(!tx.is_valid());

        // Sign it
        tx.sign(&wallet).unwrap();

        // Now it should be valid
        assert!(tx.is_valid());
    }

    #[test]
    fn test_coinbase_transaction() {
        let coinbase = CoinbaseTransaction::new(
            "AURQ1234".to_string(),
            5_000_000_000, // 50 AUR reward
            100_000_000,   // 1 AUR in fees
            1
        );

        assert_eq!(coinbase.block_height, 1);
        assert_eq!(coinbase.total_reward(), 5_100_000_000);
    }
}
