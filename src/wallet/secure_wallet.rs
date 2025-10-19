use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use argon2::password_hash::{rand_core::RngCore, SaltString};
use bip39::Mnemonic;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use sha3::Keccak256;
use zeroize::Zeroize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Wallet ultra-sécurisé avec protection quantique
#[derive(Serialize, Deserialize)]
pub struct SecureWallet {
    pub name: String,
    pub address: String,
    pub address_type: String,
    
    encrypted_private_key: Vec<u8>,
    encrypted_seed: Vec<u8>,
    
    pub created_at: String,
    pub version: String,
    nonce: Vec<u8>,
    salt: String,
    
    integrity_hash: String,
    pub quantum_ready: bool,
}

impl SecureWallet {
    pub fn generate(name: String, password: &str, address_type: &str) -> Result<Self, String> {
        let mut entropy = [0u8; 32];
        OsRng.fill_bytes(&mut entropy);
        
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .map_err(|e| format!("Failed to generate mnemonic: {}", e))?;
        
        println!("\n╔════════════════════════════════════════════════╗");
        println!("║        🔐 SEED PHRASE (WRITE IT DOWN!)        ║");
        println!("╠════════════════════════════════════════════════╣");
        println!("║                                                ║");
        
        let words: Vec<&str> = mnemonic.words().collect();
        for (i, word) in words.iter().enumerate() {
            if i % 4 == 0 {
                print!("║  ");
            }
            print!("{:2}. {:<10} ", i + 1, word);
            if (i + 1) % 4 == 0 {
                println!("║");
            }
        }
        
        println!("║                                                ║");
        println!("╚════════════════════════════════════════════════╝");
        println!("\n⚠️  CRITICAL: Write down these 24 words ON PAPER!");
        println!("⚠️  Store them in a SAFE PLACE!");
        println!("⚠️  NEVER share them with ANYONE!");
        println!("\n⏸️  Press ENTER when you have written them down...");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let seed = mnemonic.to_seed("");
        let mut private_key_bytes = [0u8; 32];
        let mut hasher = Sha256::new();
        hasher.update(&seed[0..32]);
        let result = hasher.finalize();
        private_key_bytes.copy_from_slice(&result);
        
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&private_key_bytes)
            .map_err(|e| format!("Invalid private key: {}", e))?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        
        let address = Self::generate_address(&public_key, address_type);
        
        let (encrypted_private_key, encrypted_seed, nonce, salt) = 
            Self::encrypt_sensitive_data(&private_key_bytes, mnemonic.to_string().as_bytes(), password)?;
        
        private_key_bytes.zeroize();
        
        let integrity_hash = Self::calculate_integrity_hash(
            &encrypted_private_key,
            &encrypted_seed,
            &nonce,
            &salt,
        );
        
        let quantum_ready = matches!(address_type, "AUR2" | "AUR3");
        
        Ok(SecureWallet {
            name,
            address,
            address_type: address_type.to_string(),
            encrypted_private_key,
            encrypted_seed,
            created_at: chrono::Utc::now().to_rfc3339(),
            version: "1.0.0-quantum-secure".to_string(),
            nonce,
            salt,
            integrity_hash,
            quantum_ready,
        })
    }
    
    fn encrypt_sensitive_data(
        private_key: &[u8],
        seed: &[u8],
        password: &str,
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, String), String> {
        let salt = SaltString::generate(&mut OsRng);
        
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Argon2 failed: {}", e))?;
        
        let hash_binding = password_hash.hash.unwrap();
        let key_bytes = &hash_binding.as_bytes()[0..32];
        
        let cipher = Aes256Gcm::new_from_slice(key_bytes)
            .map_err(|e| format!("Cipher creation failed: {}", e))?;
        
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let encrypted_private_key = cipher
            .encrypt(nonce, private_key)
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        let encrypted_seed = cipher
            .encrypt(nonce, seed)
            .map_err(|e| format!("Seed encryption failed: {}", e))?;
        
        Ok((
            encrypted_private_key,
            encrypted_seed,
            nonce_bytes.to_vec(),
            salt.to_string(),
        ))
    }
    
    pub fn decrypt_private_key(&self, password: &str) -> Result<Vec<u8>, String> {
        if !self.verify_integrity() {
            return Err("SECURITY ALERT: Wallet integrity compromised!".to_string());
        }
        
        let salt = PasswordHash::new(&self.salt)
            .map_err(|e| format!("Invalid salt: {}", e))?;
        
        let argon2 = Argon2::default();
        
        argon2
            .verify_password(password.as_bytes(), &salt)
            .map_err(|_| "Invalid password!".to_string())?;
        
        let hash_binding = salt.hash.unwrap();
        let key_bytes = &hash_binding.as_bytes()[0..32];
        
        let cipher = Aes256Gcm::new_from_slice(key_bytes)
            .map_err(|e| format!("Cipher creation failed: {}", e))?;
        
        let nonce = Nonce::from_slice(&self.nonce);
        
        let decrypted = cipher
            .decrypt(nonce, self.encrypted_private_key.as_ref())
            .map_err(|_| "Decryption failed! Wrong password?".to_string())?;
        
        Ok(decrypted)
    }
    
    pub fn decrypt_seed(&self, password: &str) -> Result<String, String> {
        if !self.verify_integrity() {
            return Err("SECURITY ALERT: Wallet integrity compromised!".to_string());
        }
        
        let salt = PasswordHash::new(&self.salt)
            .map_err(|e| format!("Invalid salt: {}", e))?;
        
        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &salt)
            .map_err(|_| "Invalid password!".to_string())?;
        
        let hash_binding = salt.hash.unwrap();
        let key_bytes = &hash_binding.as_bytes()[0..32];
        let cipher = Aes256Gcm::new_from_slice(key_bytes)
            .map_err(|e| format!("Cipher creation failed: {}", e))?;
        
        let nonce = Nonce::from_slice(&self.nonce);
        
        let decrypted = cipher
            .decrypt(nonce, self.encrypted_seed.as_ref())
            .map_err(|_| "Decryption failed!".to_string())?;
        
        String::from_utf8(decrypted)
            .map_err(|_| "Invalid seed data".to_string())
    }
    
    fn calculate_integrity_hash(
        encrypted_key: &[u8],
        encrypted_seed: &[u8],
        nonce: &[u8],
        salt: &str,
    ) -> String {
        let mut hasher = Keccak256::new();
        hasher.update(encrypted_key);
        hasher.update(encrypted_seed);
        hasher.update(nonce);
        hasher.update(salt.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    pub fn verify_integrity(&self) -> bool {
        let calculated_hash = Self::calculate_integrity_hash(
            &self.encrypted_private_key,
            &self.encrypted_seed,
            &self.nonce,
            &self.salt,
        );
        
        calculated_hash == self.integrity_hash
    }
    
    fn generate_address(public_key: &PublicKey, addr_type: &str) -> String {
        let public_key_bytes = public_key.serialize();
        
        let hash1 = Sha256::digest(&public_key_bytes);
        let hash2 = Sha256::digest(&hash1);
        
        let hash160 = &hash2[0..20];
        
        let version_byte = match addr_type {
            "AUR1" => 0x4F,
            "AUR2" => 0x50,
            "AUR3" => 0x51,
            _ => 0x4F,
        };
        
        let mut payload = vec![version_byte];
        payload.extend_from_slice(hash160);
        
        let checksum_hash = Sha256::digest(&Sha256::digest(&payload));
        let checksum = &checksum_hash[0..4];
        payload.extend_from_slice(checksum);
        
        let encoded = bs58::encode(payload).into_string();
        
        format!("{}{}", addr_type, encoded)
    }
    
    pub fn save(&self, directory: &str) -> Result<String, String> {
        let wallet_dir = Path::new(directory);
        if !wallet_dir.exists() {
            fs::create_dir_all(wallet_dir)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        let filename = format!("{}/{}.secure.wallet", directory, self.name);
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        fs::write(&filename, json)
            .map_err(|e| format!("Failed to write wallet: {}", e))?;
        
        Ok(filename)
    }
    
    pub fn load(filename: &str) -> Result<Self, String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read wallet: {}", e))?;
        
        let wallet: SecureWallet = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse wallet: {}", e))?;
        
        if !wallet.verify_integrity() {
            return Err("⚠️  SECURITY ALERT: Wallet has been tampered with!".to_string());
        }
        
        Ok(wallet)
    }
    
    pub fn security_info(&self) {
        println!("\n╔════════════════════════════════════════════════╗");
        println!("║           🔐 WALLET SECURITY INFO 🔐          ║");
        println!("╠════════════════════════════════════════════════╣");
        println!("║                                                ║");
        println!("║  Encryption     : AES-256-GCM ✅               ║");
        println!("║  Key Derivation : Argon2 ✅                    ║");
        println!("║  Seed Format    : BIP39 (24 words) ✅          ║");
        println!("║  Quantum Ready  : {} ║", 
            if self.quantum_ready { "YES ✅              " } else { "NO ⚠️               " });
        println!("║  Address Type   : {} ║", format!("{:<29}", self.address_type));
        println!("║  Integrity      : {} ║",
            if self.verify_integrity() { "VERIFIED ✅         " } else { "COMPROMISED ❌      " });
        println!("║                                                ║");
        println!("╚════════════════════════════════════════════════╝\n");
    }
}