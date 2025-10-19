use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use hex;

/// Types d'adresses supportés
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AddressType {
    Legacy,       // AUR1... (ECDSA secp256k1) - Standard actuel
    QuantumSafe,  // AUR2... (Post-quantum) - Futur
    Hybrid,       // AUR3... (ECDSA + PQ) - Transition
}

/// Version de la signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureVersion {
    V1,  // ECDSA seule (actuel)
    V2,  // Post-quantum seule (futur)
    V3,  // Hybride (transition)
}

/// Informations sur la protection quantique
#[derive(Debug, Clone)]
pub struct QuantumResistanceInfo {
    pub is_quantum_safe: bool,
    pub algorithm: String,
    pub security_level: u32,  // Bits de sécurité
    pub signature_size: usize, // Taille en bytes
}

impl AddressType {
    /// Obtenir le préfixe de l'adresse
    pub fn prefix(&self) -> &str {
        match self {
            AddressType::Legacy => "AUR1",
            AddressType::QuantumSafe => "AUR2",
            AddressType::Hybrid => "AUR3",
        }
    }

    /// Obtenir le byte de version
    pub fn version_byte(&self) -> u8 {
        match self {
            AddressType::Legacy => 0x4F,      // 'O' pour legacy
            AddressType::QuantumSafe => 0x50, // 'P' pour quantum
            AddressType::Hybrid => 0x51,      // 'Q' pour hybrid
        }
    }

    /// Parser une adresse pour déterminer son type
    pub fn from_address(address: &str) -> Option<Self> {
        if address.starts_with("AUR1") {
            Some(AddressType::Legacy)
        } else if address.starts_with("AUR2") {
            Some(AddressType::QuantumSafe)
        } else if address.starts_with("AUR3") {
            Some(AddressType::Hybrid)
        } else {
            None
        }
    }

    /// Vérifier si l'adresse est résistante aux attaques quantiques
    pub fn is_quantum_resistant(&self) -> bool {
        match self {
            AddressType::Legacy => false,
            AddressType::QuantumSafe => true,
            AddressType::Hybrid => true,
        }
    }
}

/// Gestionnaire de protection quantique
pub struct QuantumProtection {
    pub enabled: bool,
    pub algorithm: String,
}

impl QuantumProtection {
    pub fn new() -> Self {
        QuantumProtection {
            enabled: false,  // Désactivé par défaut (2025)
            algorithm: "DILITHIUM3".to_string(), // Algorithme NIST recommandé
        }
    }

    /// Vérifier si la protection quantique doit être activée
    pub fn should_activate(&self, year: i32) -> bool {
        // Activer automatiquement après 2030
        year >= 2030
    }

    /// Obtenir les informations de sécurité pour un type d'adresse
    pub fn get_security_info(&self, addr_type: AddressType) -> QuantumResistanceInfo {
        match addr_type {
            AddressType::Legacy => QuantumResistanceInfo {
                is_quantum_safe: false,
                algorithm: "ECDSA-secp256k1".to_string(),
                security_level: 128,  // 128 bits contre attaques classiques
                signature_size: 70,   // ~70 bytes
            },
            AddressType::QuantumSafe => QuantumResistanceInfo {
                is_quantum_safe: true,
                algorithm: "DILITHIUM3".to_string(),
                security_level: 192,  // 192 bits contre attaques quantiques
                signature_size: 2420, // ~2.4 KB
            },
            AddressType::Hybrid => QuantumResistanceInfo {
                is_quantum_safe: true,
                algorithm: "ECDSA+DILITHIUM3".to_string(),
                security_level: 192,  // Maximum des deux
                signature_size: 2490, // ECDSA + DILITHIUM
            },
        }
    }

    /// Recommandation de type d'adresse selon l'année
    pub fn recommend_address_type(&self, year: i32) -> AddressType {
        if year < 2028 {
            AddressType::Legacy  // Avant 2028 : Legacy suffit
        } else if year < 2033 {
            AddressType::Hybrid  // 2028-2033 : Transition vers hybride
        } else {
            AddressType::QuantumSafe  // Après 2033 : Quantum-safe obligatoire
        }
    }

    /// Message d'avertissement si nécessaire
    pub fn get_warning(&self, addr_type: AddressType, year: i32) -> Option<String> {
        if year >= 2030 && addr_type == AddressType::Legacy {
            Some(format!(
                "⚠️  AVERTISSEMENT: Les adresses Legacy (AUR1) seront vulnérables \
                aux ordinateurs quantiques après 2035. \
                Considérez migrer vers une adresse Quantum-Safe (AUR2) ou Hybrid (AUR3)."
            ))
        } else {
            None
        }
    }
}

impl Default for QuantumProtection {
    fn default() -> Self {
        Self::new()
    }
}

/// Utilitaire pour générer des adresses avec type
pub struct AddressGenerator;

impl AddressGenerator {
    /// Générer une adresse avec un type spécifique
    pub fn generate_with_type(public_key: &[u8], addr_type: AddressType) -> String {
        // Hash de la clé publique
        let hash1 = Sha256::digest(public_key);
        let hash2 = Sha256::digest(&hash1);
        
        // Prendre les 20 premiers bytes
        let hash160 = &hash2[0..20];
        
        // Ajouter le byte de version
        let mut payload = vec![addr_type.version_byte()];
        payload.extend_from_slice(hash160);
        
        // Calculer le checksum
        let checksum_hash = Sha256::digest(&Sha256::digest(&payload));
        let checksum = &checksum_hash[0..4];
        payload.extend_from_slice(checksum);
        
        // Encoder en Base58
        let encoded = bs58::encode(payload).into_string();
        
        // Ajouter le préfixe
        format!("{}{}", addr_type.prefix(), encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_type_detection() {
        assert_eq!(AddressType::from_address("AUR1qxyz..."), Some(AddressType::Legacy));
        assert_eq!(AddressType::from_address("AUR2qabc..."), Some(AddressType::QuantumSafe));
        assert_eq!(AddressType::from_address("AUR3qdef..."), Some(AddressType::Hybrid));
        assert_eq!(AddressType::from_address("BTC1..."), None);
    }

    #[test]
    fn test_quantum_resistance() {
        assert!(!AddressType::Legacy.is_quantum_resistant());
        assert!(AddressType::QuantumSafe.is_quantum_resistant());
        assert!(AddressType::Hybrid.is_quantum_resistant());
    }

    #[test]
    fn test_recommendation_by_year() {
        let qp = QuantumProtection::new();
        
        assert_eq!(qp.recommend_address_type(2025), AddressType::Legacy);
        assert_eq!(qp.recommend_address_type(2030), AddressType::Hybrid);
        assert_eq!(qp.recommend_address_type(2035), AddressType::QuantumSafe);
    }

    #[test]
    fn test_security_info() {
        let qp = QuantumProtection::new();
        
        let legacy_info = qp.get_security_info(AddressType::Legacy);
        assert_eq!(legacy_info.is_quantum_safe, false);
        assert_eq!(legacy_info.security_level, 128);
        
        let quantum_info = qp.get_security_info(AddressType::QuantumSafe);
        assert_eq!(quantum_info.is_quantum_safe, true);
        assert_eq!(quantum_info.security_level, 192);
    }

    #[test]
    fn test_warning_message() {
        let qp = QuantumProtection::new();
        
        // Pas d'avertissement en 2025
        assert!(qp.get_warning(AddressType::Legacy, 2025).is_none());
        
        // Avertissement en 2030
        assert!(qp.get_warning(AddressType::Legacy, 2030).is_some());
    }
}
