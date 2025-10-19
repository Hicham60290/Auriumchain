use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

pub fn generate_address(public_key: &[u8]) -> String {
    // Double SHA-256
    let hash1 = Sha256::digest(public_key);
    let hash2 = Sha256::digest(&hash1);
    
    // RIPEMD-160
    let hash160 = Ripemd160::digest(&hash2);
    
    // Ajouter préfixe réseau (AUR1)
    let mut payload = vec![0x4F]; // 'O' en ASCII = AUR1
    payload.extend_from_slice(&hash160);
    
    // Calculer checksum (4 premiers bytes du double SHA-256)
    let checksum_hash = Sha256::digest(&Sha256::digest(&payload));
    let checksum = &checksum_hash[0..4];
    payload.extend_from_slice(checksum);
    
    // Encoder en Base58
    let encoded = bs58::encode(payload).into_string();
    
    format!("AUR1{}", encoded)
}

pub fn validate_address(address: &str) -> bool {
    // Vérifier le préfixe (AUR1, AUR2 ou AUR3)
    if !address.starts_with("AUR1") && !address.starts_with("AUR2") && !address.starts_with("AUR3") {
        return false;
    }

    // Extraire la partie après le préfixe
    let addr_without_prefix = if address.starts_with("AUR1") {
        &address[4..]
    } else if address.starts_with("AUR2") {
        &address[4..]
    } else if address.starts_with("AUR3") {
        &address[4..]
    } else {
        return false;
    };

    // Décoder Base58
    let decoded = match bs58::decode(addr_without_prefix).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };

    // Doit avoir au moins 25 bytes (1 version + 20 hash + 4 checksum)
    if decoded.len() < 25 {
        return false;
    }

    // Séparer payload et checksum
    let checksum_index = decoded.len() - 4;
    let payload = &decoded[..checksum_index];
    let checksum = &decoded[checksum_index..];

    // Vérifier le checksum
    let hash = Sha256::digest(&Sha256::digest(payload));
    let expected_checksum = &hash[0..4];

    checksum == expected_checksum
}

pub fn address_from_keypair(keypair: &crate::wallet::keys::KeyPair) -> String {
    let public_key = keypair.public_key.serialize();
    generate_address(&public_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_generation() {
        let public_key = vec![
            0x02, 0x9b, 0x6c, 0x4d, 0x7e, 0x8a, 0x9f, 0x1b,
            0x2c, 0x3d, 0x4e, 0x5f, 0x6a, 0x7b, 0x8c, 0x9d,
            0x0e, 0x1f, 0x2a, 0x3b, 0x4c, 0x5d, 0x6e, 0x7f,
            0x8a, 0x9b, 0x0c, 0x1d, 0x2e, 0x3f, 0x4a, 0x5b,
            0x6c
        ];
        
        let address = generate_address(&public_key);
        
        assert!(address.starts_with("AUR1"));
        assert!(validate_address(&address));
    }

    #[test]
    fn test_invalid_address() {
        assert!(!validate_address("BTC1qxyz..."));
        assert!(!validate_address("invalid"));
        assert!(!validate_address(""));
    }
}