use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::rngs::OsRng;
use sha2::Digest;

pub struct KeyPair {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

impl KeyPair {
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng;
        
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        
        KeyPair {
            private_key: secret_key,
            public_key,
        }
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public_key.serialize())
    }
}
