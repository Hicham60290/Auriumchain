use crate::blockchain::Transaction;
use anyhow::{Result, anyhow};
use secp256k1::{Secp256k1, Message, ecdsa::Signature};
use sha2::{Sha256, Digest};

pub struct SignatureValidator {
    secp: Secp256k1<secp256k1::All>,
}

impl SignatureValidator {
    pub fn new() -> Self {
        SignatureValidator {
            secp: Secp256k1::new(),
        }
    }

    /// Verify all signatures in a transaction
    pub fn verify_transaction(&self, tx: &Transaction) -> Result<()> {
        // Coinbase transactions don't need signature verification
        if tx.is_coinbase() {
            return Ok(());
        }

        // Every input must have a valid signature
        for (idx, input) in tx.inputs.iter().enumerate() {
            self.verify_input_signature(tx, idx)?;
        }

        Ok(())
    }

    /// Verify signature for a specific input
    fn verify_input_signature(&self, tx: &Transaction, input_index: usize) -> Result<()> {
        let input = &tx.inputs[input_index];

        // Parse public key
        let pubkey_bytes = hex::decode(&input.public_key)
            .map_err(|_| anyhow!("Invalid public key hex"))?;
        let pubkey = secp256k1::PublicKey::from_slice(&pubkey_bytes)
            .map_err(|_| anyhow!("Invalid public key format"))?;

        // Parse signature
        let sig_bytes = hex::decode(&input.signature)
            .map_err(|_| anyhow!("Invalid signature hex"))?;
        let signature = Signature::from_compact(&sig_bytes)
            .map_err(|_| anyhow!("Invalid signature format"))?;

        // Create signing message (hash of transaction data without signatures)
        let message_hash = self.create_signing_hash(tx, input_index)?;
        let message = Message::from_slice(&message_hash)
            .map_err(|_| anyhow!("Invalid message hash"))?;

        // Verify signature
        self.secp.verify_ecdsa(&message, &signature, &pubkey)
            .map_err(|_| anyhow!("Signature verification failed for input {}", input_index))?;

        Ok(())
    }

    /// Create the hash that should be signed
    /// This includes all transaction data except signatures
    fn create_signing_hash(&self, tx: &Transaction, input_index: usize) -> Result<Vec<u8>> {
        let mut hasher = Sha256::new();

        // Include transaction ID
        hasher.update(tx.id.as_bytes());

        // Include all inputs (without signatures)
        for (idx, input) in tx.inputs.iter().enumerate() {
            hasher.update(input.prev_tx_id.as_bytes());
            hasher.update(&input.output_index.to_le_bytes());
            hasher.update(input.public_key.as_bytes());

            // Only include signature of current input being verified
            // (to prevent circular dependency)
            if idx == input_index {
                hasher.update(b"");
            }
        }

        // Include all outputs
        for output in &tx.outputs {
            hasher.update(output.address.as_bytes());
            hasher.update(&output.value.to_le_bytes());
        }

        // Include timestamp
        hasher.update(&tx.timestamp.to_le_bytes());

        Ok(hasher.finalize().to_vec())
    }

    /// Batch verify all transactions in a block
    pub fn verify_block_transactions(&self, transactions: &[Transaction]) -> Result<()> {
        for (idx, tx) in transactions.iter().enumerate() {
            self.verify_transaction(tx)
                .map_err(|e| anyhow!("Transaction {} failed validation: {}", idx, e))?;
        }
        Ok(())
    }

    /// Verify that the public key matches the expected address
    pub fn verify_pubkey_matches_address(&self, pubkey: &str, expected_address: &str) -> Result<()> {
        // Parse public key
        let pubkey_bytes = hex::decode(pubkey)
            .map_err(|_| anyhow!("Invalid public key hex"))?;
        let _parsed_pubkey = secp256k1::PublicKey::from_slice(&pubkey_bytes)
            .map_err(|_| anyhow!("Invalid public key format"))?;

        // Generate address from public key
        // TODO: Implement proper address generation from pubkey
        // For now, we trust the pubkey provided
        // This should use the same algorithm as in src/wallet/address.rs

        // Placeholder - should be replaced with actual address derivation
        if expected_address.starts_with("AUR") {
            Ok(())
        } else {
            Err(anyhow!("Invalid address format"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::{TxInput, TxOutput};

    #[test]
    fn test_signature_validator_creation() {
        let validator = SignatureValidator::new();
        assert!(true); // Just check it doesn't panic
    }

    #[test]
    fn test_coinbase_transaction_validation() {
        let validator = SignatureValidator::new();

        // Create a coinbase transaction
        let coinbase_tx = Transaction::new(
            vec![TxInput {
                prev_tx_id: "0".to_string(),
                output_index: 0,
                public_key: "coinbase".to_string(),
                signature: "".to_string(),
            }],
            vec![TxOutput {
                address: "AUR3TestAddress".to_string(),
                value: 50_000_000,
            }]
        );

        // Coinbase should pass without signature
        assert!(validator.verify_transaction(&coinbase_tx).is_ok());
    }
}
