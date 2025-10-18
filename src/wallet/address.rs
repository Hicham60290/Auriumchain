use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

pub fn generate_address(public_key: &[u8]) -> String {
    // 1. SHA-256 du public key
    let sha256_hash = Sha256::digest(public_key);
    
    // 2. RIPEMD-160
    let ripemd_hash = Ripemd160::digest(&sha256_hash);
    
    // 3. Ajouter version byte
    let mut versioned = vec![0x01]; // Version 1
    versioned.extend_from_slice(&ripemd_hash);
    
    // 4. Double SHA-256 pour checksum
    let checksum_full = Sha256::digest(&Sha256::digest(&versioned));
    let checksum = &checksum_full[0..4];
    
    // 5. Combiner
    versioned.extend_from_slice(checksum);
    
    // 6. Base58 encode
    let encoded = bs58::encode(versioned).into_string();
    
    // 7. Ajouter préfixe AUR1
    format!("AUR1{}", encoded)
}

pub fn validate_address(address: &str) -> bool {
    // Vérifier le préfixe
    if !address.starts_with("AUR1") && !address.starts_with("AURT") {
        return false;
    }
    
    // Vérifier la longueur
    if address.len() < 40 || address.len() > 50 {
        return false;
    }
    
    // Décoder et vérifier checksum
    let encoded = &address[4..];
    if let Ok(decoded) = bs58::decode(encoded).into_vec() {
        if decoded.len() < 25 {
            return false;
        }
        
        let payload = &decoded[0..decoded.len()-4];
        let checksum = &decoded[decoded.len()-4..];
        
        let checksum_verify = &Sha256::digest(&Sha256::digest(payload))[0..4];
        
        return checksum == checksum_verify;
    }
    
    false
}

pub fn address_from_keypair(keypair: &crate::wallet::keys::KeyPair) -> String {
    let public_key_bytes = keypair.public_key.serialize();
    generate_address(&public_key_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::keys::KeyPair;

    #[test]
    fn test_address_generation() {
        let keypair = KeyPair::generate();
        let address = address_from_keypair(&keypair);
        
        assert!(address.starts_with("AUR1"));
        assert!(validate_address(&address));
    }

    #[test]
    fn test_invalid_address() {
        assert!(!validate_address("BTC1invalid"));
        assert!(!validate_address("AUR1"));
        assert!(!validate_address("AUR1abc"));
    }
}
