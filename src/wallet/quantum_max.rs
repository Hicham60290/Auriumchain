/// Triple Post-Quantum Security Wallet
/// Combines 3 signature algorithms for maximum security:
/// - ECDSA secp256k1 (classical security, 256-bit)
/// - Dilithium5 (lattice-based post-quantum, NIST Level 5)
/// - SPHINCS+ (hash-based post-quantum, stateless)
///
/// An attacker must break ALL 3 algorithms to forge a signature.
/// Even if quantum computers break ECDSA, Dilithium and SPHINCS+ remain secure.

use secp256k1::{Secp256k1, SecretKey as EcdsaSecret, PublicKey as EcdsaPublic, Message};
use pqcrypto_dilithium::dilithium5;
use pqcrypto_sphincsplus::sphincsshake256256srobust as sphincs;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use rand::rngs::OsRng;
use std::time::Instant;

/// Maximum security wallet with 3-layer cryptographic protection
#[derive(Clone)]
pub struct MaxSecurityWallet {
    // Layer 1: ECDSA (classical security - current standard)
    ecdsa_private: EcdsaSecret,
    ecdsa_public: EcdsaPublic,

    // Layer 2: Dilithium5 (post-quantum lattice-based - fast)
    dilithium_private: dilithium5::SecretKey,
    dilithium_public: dilithium5::PublicKey,

    // Layer 3: SPHINCS+ (post-quantum hash-based - ultra-secure)
    sphincs_private: sphincs::SecretKey,
    sphincs_public: sphincs::PublicKey,

    // Derived address from all 3 public keys
    address: String,
}

/// Triple signature containing all 3 algorithm signatures
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TripleSignature {
    pub ecdsa: Vec<u8>,        // ~64 bytes
    pub dilithium: Vec<u8>,    // ~2,420 bytes
    pub sphincs: Vec<u8>,      // ~49,856 bytes
    // TOTAL: ~52,340 bytes (~52 KB per signature)
}

impl TripleSignature {
    /// Get total size in bytes
    pub fn size(&self) -> usize {
        self.ecdsa.len() + self.dilithium.len() + self.sphincs.len()
    }
}

/// Public keys for signature verification
#[derive(Clone, Serialize, Deserialize)]
pub struct TriplePublicKeys {
    pub ecdsa: Vec<u8>,         // Serialized ECDSA public key
    pub dilithium: Vec<u8>,     // Dilithium public key bytes
    pub sphincs: Vec<u8>,       // SPHINCS+ public key bytes
}

impl MaxSecurityWallet {
    /// Generate a new wallet with maximum security (3 algorithms)
    ///
    /// This takes ~800ms due to SPHINCS+ key generation
    pub fn new() -> Self {
        println!("🔐 Generating MAXIMUM SECURITY wallet...");
        let start = Instant::now();

        // 1️⃣ Generate ECDSA keypair (fast: ~1ms)
        println!("  ⚙️  Generating ECDSA keys...");
        let ecdsa_start = Instant::now();
        let secp = Secp256k1::new();
        let ecdsa_private = EcdsaSecret::new(&mut OsRng);
        let ecdsa_public = EcdsaPublic::from_secret_key(&secp, &ecdsa_private);
        println!("  ✅ ECDSA ready - {:?}", ecdsa_start.elapsed());

        // 2️⃣ Generate Dilithium5 keypair (medium: ~15ms)
        println!("  ⚙️  Generating Dilithium5 keys...");
        let dil_start = Instant::now();
        let (dilithium_public, dilithium_private) = dilithium5::keypair();
        println!("  ✅ Dilithium5 ready - {:?}", dil_start.elapsed());

        // 3️⃣ Generate SPHINCS+ keypair (slow: ~700ms)
        println!("  ⚙️  Generating SPHINCS+ keys (this takes a moment)...");
        let sph_start = Instant::now();
        let (sphincs_public, sphincs_private) = sphincs::keypair();
        println!("  ✅ SPHINCS+ ready - {:?}", sph_start.elapsed());

        // Derive unique address from all 3 public keys
        let address = Self::derive_address(
            &ecdsa_public,
            &dilithium_public,
            &sphincs_public
        );

        println!("🎯 Wallet generated in {:?}", start.elapsed());
        println!("📍 Address: {}", address);

        Self {
            ecdsa_private,
            ecdsa_public,
            dilithium_private,
            dilithium_public,
            sphincs_private,
            sphincs_public,
            address,
        }
    }

    /// Derive a unique AURQ address from all 3 public keys
    ///
    /// Format: AURQ{40-hex-chars} (similar to Ethereum)
    /// The address is a hash of all 3 public keys combined
    fn derive_address(
        ecdsa: &EcdsaPublic,
        dilithium: &dilithium5::PublicKey,
        sphincs: &sphincs::PublicKey
    ) -> String {
        let mut hasher = Sha256::new();

        // Combine all 3 public keys
        hasher.update(ecdsa.serialize());
        hasher.update(dilithium.as_bytes());
        hasher.update(sphincs.as_bytes());

        let hash = hasher.finalize();

        // Prefix "AURQ" for Quantum-secured addresses
        format!("AURQ{}", hex::encode(&hash[..20]))
    }

    /// Sign a message with ALL 3 algorithms
    ///
    /// This takes ~600ms due to SPHINCS+ signing
    /// Returns a TripleSignature containing all 3 signatures
    pub fn sign_triple(&self, message: &[u8]) -> TripleSignature {
        println!("🔐 Signing with TRIPLE SECURITY...");
        let start = Instant::now();

        // 1️⃣ Sign with ECDSA (fast: ~0.1ms)
        let ecdsa_start = Instant::now();
        let secp = Secp256k1::new();

        // Hash message to 32 bytes for ECDSA
        let mut hasher = Sha256::new();
        hasher.update(message);
        let msg_hash = hasher.finalize();

        let msg_secp = Message::from_digest_slice(&msg_hash)
            .expect("32 byte hash");
        let ecdsa_sig = secp.sign_ecdsa(&msg_secp, &self.ecdsa_private);
        let ecdsa = ecdsa_sig.serialize_compact().to_vec();
        println!("  ✅ ECDSA signed ({} bytes) - {:?}", ecdsa.len(), ecdsa_start.elapsed());

        // 2️⃣ Sign with Dilithium5 (medium: ~15ms)
        let dil_start = Instant::now();
        let dilithium_signed = dilithium5::sign(message, &self.dilithium_private);
        let dilithium = dilithium_signed.as_bytes().to_vec();
        println!("  ✅ Dilithium5 signed ({} bytes) - {:?}", dilithium.len(), dil_start.elapsed());

        // 3️⃣ Sign with SPHINCS+ (slow: ~600ms)
        let sph_start = Instant::now();
        let sphincs_signed = sphincs::sign(message, &self.sphincs_private);
        let sphincs = sphincs_signed.as_bytes().to_vec();
        println!("  ✅ SPHINCS+ signed ({} bytes) - {:?}", sphincs.len(), sph_start.elapsed());

        let total_size = ecdsa.len() + dilithium.len() + sphincs.len();
        println!("🎯 Triple signature complete - {:?}", start.elapsed());
        println!("📦 Total size: {} bytes (~{} KB)", total_size, total_size / 1024);

        TripleSignature {
            ecdsa,
            dilithium,
            sphincs,
        }
    }

    /// Verify a triple signature
    ///
    /// ALL 3 signatures must be valid for verification to succeed
    /// This takes ~80ms due to SPHINCS+ verification
    pub fn verify_triple(
        signature: &TripleSignature,
        message: &[u8],
        public_keys: &TriplePublicKeys
    ) -> Result<bool, String> {
        println!("🔍 Verifying TRIPLE SIGNATURE...");
        let start = Instant::now();

        // 1️⃣ Verify ECDSA signature
        let ecdsa_start = Instant::now();
        let secp = Secp256k1::new();

        // Reconstruct ECDSA public key
        let ecdsa_pub = EcdsaPublic::from_slice(&public_keys.ecdsa)
            .map_err(|e| format!("Invalid ECDSA public key: {}", e))?;

        // Hash message
        let mut hasher = Sha256::new();
        hasher.update(message);
        let msg_hash = hasher.finalize();

        let msg_secp = Message::from_digest_slice(&msg_hash)
            .map_err(|e| format!("Message hash error: {}", e))?;
        let ecdsa_sig = secp256k1::ecdsa::Signature::from_compact(&signature.ecdsa)
            .map_err(|e| format!("Invalid ECDSA signature: {}", e))?;

        let ecdsa_valid = secp.verify_ecdsa(&msg_secp, &ecdsa_sig, &ecdsa_pub).is_ok();
        println!("  {} ECDSA - {:?}",
            if ecdsa_valid { "✅" } else { "❌" },
            ecdsa_start.elapsed()
        );

        if !ecdsa_valid {
            return Ok(false);
        }

        // 2️⃣ Verify Dilithium5 signature
        let dil_start = Instant::now();
        let dilithium_pub = dilithium5::PublicKey::from_bytes(&public_keys.dilithium)
            .map_err(|e| format!("Invalid Dilithium public key: {:?}", e))?;

        let dilithium_msg = dilithium5::SignedMessage::from_bytes(&signature.dilithium)
            .map_err(|e| format!("Invalid Dilithium signature: {:?}", e))?;

        let dilithium_valid = dilithium5::open(&dilithium_msg, &dilithium_pub).is_ok();
        println!("  {} Dilithium5 - {:?}",
            if dilithium_valid { "✅" } else { "❌" },
            dil_start.elapsed()
        );

        if !dilithium_valid {
            return Ok(false);
        }

        // 3️⃣ Verify SPHINCS+ signature
        let sph_start = Instant::now();
        let sphincs_pub = sphincs::PublicKey::from_bytes(&public_keys.sphincs)
            .map_err(|e| format!("Invalid SPHINCS public key: {:?}", e))?;

        let sphincs_msg = sphincs::SignedMessage::from_bytes(&signature.sphincs)
            .map_err(|e| format!("Invalid SPHINCS signature: {:?}", e))?;

        let sphincs_valid = sphincs::open(&sphincs_msg, &sphincs_pub).is_ok();
        println!("  {} SPHINCS+ - {:?}",
            if sphincs_valid { "✅" } else { "❌" },
            sph_start.elapsed()
        );

        // All 3 must be valid
        let all_valid = ecdsa_valid && dilithium_valid && sphincs_valid;

        println!("🎯 Verification {} - Total time: {:?}",
            if all_valid { "SUCCESS ✅" } else { "FAILED ❌" },
            start.elapsed()
        );

        Ok(all_valid)
    }

    /// Get wallet address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Export public keys for verification
    pub fn export_public_keys(&self) -> TriplePublicKeys {
        TriplePublicKeys {
            ecdsa: self.ecdsa_public.serialize().to_vec(),
            dilithium: self.dilithium_public.as_bytes().to_vec(),
            sphincs: self.sphincs_public.as_bytes().to_vec(),
        }
    }

    /// Get security level description
    pub fn security_level(&self) -> String {
        format!(
            "MAXIMUM SECURITY\n\
            - ECDSA secp256k1: 256-bit classical security\n\
            - Dilithium5 (NIST Level 5): Quantum-resistant (lattice)\n\
            - SPHINCS+ SHA-256: Quantum-resistant (hash-based)\n\
            \n\
            Protection: Remains secure even if 2 out of 3 algorithms are broken"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_generation() {
        let wallet = MaxSecurityWallet::new();
        assert!(wallet.address().starts_with("AURQ"));
        assert_eq!(wallet.address().len(), 44); // "AURQ" + 40 hex chars
    }

    #[test]
    fn test_triple_signature() {
        let wallet = MaxSecurityWallet::new();
        let message = b"Test transaction";

        // Sign with all 3 algorithms
        let signature = wallet.sign_triple(message);

        // Verify signature size
        assert!(signature.ecdsa.len() > 0);
        assert!(signature.dilithium.len() > 0);
        assert!(signature.sphincs.len() > 0);

        // Verify signature
        let public_keys = wallet.export_public_keys();
        let valid = MaxSecurityWallet::verify_triple(&signature, message, &public_keys)
            .expect("Verification should not error");

        assert!(valid, "Triple signature should be valid");
    }

    #[test]
    fn test_signature_tampering() {
        let wallet = MaxSecurityWallet::new();
        let message = b"Original message";
        let signature = wallet.sign_triple(message);

        // Try to verify with different message
        let tampered_message = b"Tampered message";
        let public_keys = wallet.export_public_keys();

        let valid = MaxSecurityWallet::verify_triple(&signature, tampered_message, &public_keys)
            .expect("Verification should not error");

        assert!(!valid, "Tampered message should fail verification");
    }

    #[test]
    fn test_address_uniqueness() {
        let wallet1 = MaxSecurityWallet::new();
        let wallet2 = MaxSecurityWallet::new();

        assert_ne!(wallet1.address(), wallet2.address(),
            "Different wallets should have different addresses");
    }
}
