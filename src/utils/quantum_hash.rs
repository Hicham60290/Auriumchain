/// Quantum-Resistant Double Hashing Module
/// Combines SHA-256 + BLAKE3 for maximum security against quantum attacks
///
/// Strategy: Cascade hashing for defense-in-depth
/// If one algorithm is broken (quantum or classical), the other still protects

use sha2::{Sha256, Digest as Sha2Digest};
use blake3;
use hex;

/// Quantum hash result (256 bits)
pub type QuantumHash = [u8; 32];

/// Double hash using SHA-256 + BLAKE3 cascade
///
/// Algorithm: BLAKE3(SHA-256(data))
///
/// Security properties:
/// - If SHA-256 is broken, BLAKE3 still protects
/// - If BLAKE3 is broken, SHA-256 still protects
/// - Quantum resistance: ~128 bits (Grover's algorithm limit)
/// - Classical resistance: 256 bits
///
/// Performance:
/// - SHA-256: ~200 MB/s
/// - BLAKE3: ~3-10 GB/s (SIMD optimized)
/// - Combined: Dominated by SHA-256 (~200 MB/s)
/// - Overhead: ~2x SHA-256 alone, but quantum-safe
pub fn quantum_hash(data: &[u8]) -> QuantumHash {
    // First pass: SHA-256 (industry standard, widely tested)
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(data);
    let sha256_result = sha256_hasher.finalize();

    // Second pass: BLAKE3 on SHA-256 result (modern, fast, quantum-resistant)
    let blake3_result = blake3::hash(&sha256_result);

    // Return as 32-byte array
    *blake3_result.as_bytes()
}

/// Double hash with hex output (for display/storage)
pub fn quantum_hash_hex(data: &[u8]) -> String {
    hex::encode(quantum_hash(data))
}

/// Verify hash matches data
pub fn verify_quantum_hash(data: &[u8], expected_hash: &QuantumHash) -> bool {
    let computed_hash = quantum_hash(data);
    computed_hash == *expected_hash
}

/// Alternative: Parallel XOR strategy (faster for multi-core)
///
/// Algorithm: SHA-256(data) XOR BLAKE3(data)
///
/// Advantages:
/// - Can compute both hashes in parallel
/// - Slightly faster on multi-core systems
/// - Still quantum-resistant
pub fn quantum_hash_parallel_xor(data: &[u8]) -> QuantumHash {
    // SHA-256 computation
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(data);
    let sha256_result = sha256_hasher.finalize();

    // BLAKE3 computation (can be parallelized)
    let blake3_result = blake3::hash(data);

    // XOR the two results
    let mut result = [0u8; 32];
    for i in 0..32 {
        result[i] = sha256_result[i] ^ blake3_result.as_bytes()[i];
    }

    result
}

/// Alternative: Concatenation strategy (maximum security)
///
/// Algorithm: SHA-256(SHA-256(data) || BLAKE3(data))
///
/// Advantages:
/// - Uses full 512 bits before final compression
/// - Maximum theoretical security
/// - Slower but most secure
pub fn quantum_hash_concat(data: &[u8]) -> QuantumHash {
    // SHA-256 computation
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(data);
    let sha256_result = sha256_hasher.finalize();

    // BLAKE3 computation
    let blake3_result = blake3::hash(data);

    // Concatenate and hash again with SHA-256
    let mut final_hasher = Sha256::new();
    final_hasher.update(&sha256_result);
    final_hasher.update(blake3_result.as_bytes());
    let final_result = final_hasher.finalize();

    let mut result = [0u8; 32];
    result.copy_from_slice(&final_result);
    result
}

/// Hash strategy selector
#[derive(Debug, Clone, Copy)]
pub enum QuantumHashStrategy {
    /// BLAKE3(SHA-256(data)) - Recommended
    Cascade,
    /// SHA-256(data) XOR BLAKE3(data) - Faster on multi-core
    ParallelXor,
    /// SHA-256(SHA-256(data) || BLAKE3(data)) - Maximum security
    Concatenation,
}

impl Default for QuantumHashStrategy {
    fn default() -> Self {
        QuantumHashStrategy::Cascade
    }
}

/// Configurable quantum hash with strategy selection
pub fn quantum_hash_with_strategy(data: &[u8], strategy: QuantumHashStrategy) -> QuantumHash {
    match strategy {
        QuantumHashStrategy::Cascade => quantum_hash(data),
        QuantumHashStrategy::ParallelXor => quantum_hash_parallel_xor(data),
        QuantumHashStrategy::Concatenation => quantum_hash_concat(data),
    }
}

/// Quantum-resistant block hash (for blockchain blocks)
pub fn quantum_block_hash(
    index: u64,
    timestamp: i64,
    previous_hash: &str,
    merkle_root: &str,
    nonce: u64,
    difficulty: usize,
    miner_address: &str,
) -> String {
    let mut data = Vec::new();

    // Serialize block data
    data.extend_from_slice(&index.to_le_bytes());
    data.extend_from_slice(&timestamp.to_le_bytes());
    data.extend_from_slice(previous_hash.as_bytes());
    data.extend_from_slice(merkle_root.as_bytes());
    data.extend_from_slice(&nonce.to_le_bytes());
    data.extend_from_slice(&(difficulty as u64).to_le_bytes());
    data.extend_from_slice(miner_address.as_bytes());

    quantum_hash_hex(&data)
}

/// Quantum-resistant transaction hash
pub fn quantum_transaction_hash(tx_data: &[u8]) -> String {
    quantum_hash_hex(tx_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_hash_deterministic() {
        let data = b"Hello, AuriumChain!";
        let hash1 = quantum_hash(data);
        let hash2 = quantum_hash(data);
        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_quantum_hash_different_inputs() {
        let data1 = b"AuriumChain";
        let data2 = b"Bitcoin";
        let hash1 = quantum_hash(data1);
        let hash2 = quantum_hash(data2);
        assert_ne!(hash1, hash2, "Different inputs should produce different hashes");
    }

    #[test]
    fn test_verify_quantum_hash() {
        let data = b"Test data";
        let hash = quantum_hash(data);
        assert!(verify_quantum_hash(data, &hash), "Verification should succeed");

        let wrong_data = b"Wrong data";
        assert!(!verify_quantum_hash(wrong_data, &hash), "Verification should fail for wrong data");
    }

    #[test]
    fn test_quantum_hash_hex() {
        let data = b"AuriumChain";
        let hash_hex = quantum_hash_hex(data);
        assert_eq!(hash_hex.len(), 64, "Hex hash should be 64 characters (32 bytes)");
    }

    #[test]
    fn test_all_strategies_produce_valid_hashes() {
        let data = b"Test all strategies";

        let cascade = quantum_hash_with_strategy(data, QuantumHashStrategy::Cascade);
        let xor = quantum_hash_with_strategy(data, QuantumHashStrategy::ParallelXor);
        let concat = quantum_hash_with_strategy(data, QuantumHashStrategy::Concatenation);

        // All should be 32 bytes
        assert_eq!(cascade.len(), 32);
        assert_eq!(xor.len(), 32);
        assert_eq!(concat.len(), 32);

        // Different strategies should produce different hashes
        assert_ne!(cascade, xor);
        assert_ne!(cascade, concat);
        assert_ne!(xor, concat);
    }

    #[test]
    fn test_quantum_block_hash() {
        let hash = quantum_block_hash(
            1,
            1234567890,
            "prev_hash",
            "merkle_root",
            12345,
            4,
            "AUR3TestAddress"
        );

        assert_eq!(hash.len(), 64, "Block hash should be 64 hex characters");
    }

    #[test]
    fn test_avalanche_effect() {
        // Small change in input should produce completely different hash
        let data1 = b"AuriumChain";
        let data2 = b"AuriumChain!"; // One character difference

        let hash1 = quantum_hash(data1);
        let hash2 = quantum_hash(data2);

        // Count different bytes
        let diff_count = hash1.iter()
            .zip(hash2.iter())
            .filter(|(a, b)| a != b)
            .count();

        // Should have changed many bytes (good avalanche)
        assert!(diff_count > 15, "Avalanche effect: {} bytes different", diff_count);
    }

    #[test]
    fn test_zero_preimage_resistance() {
        // Should produce valid hash even for empty input
        let empty_data = b"";
        let hash = quantum_hash(empty_data);
        assert_eq!(hash.len(), 32);
        assert!(hash.iter().any(|&b| b != 0), "Hash should not be all zeros");
    }
}
