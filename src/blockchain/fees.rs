use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub low: u64,      // Frais bas (lent)
    pub medium: u64,   // Frais moyen (normal)
    pub high: u64,     // Frais haut (rapide)
    pub per_byte: u64, // Frais par byte
}

pub struct TransactionFees {
    pub base_fee: u64,           // Frais minimum (satoshis)
    pub per_byte_fee: u64,       // Frais par byte
    pub min_relay_fee: u64,      // Frais minimum pour relay
}

impl TransactionFees {
    pub fn new() -> Self {
        TransactionFees {
            base_fee: 1000,              // 0.00001 AUR
            per_byte_fee: 100,           // 0.000001 AUR/byte
            min_relay_fee: 500,          // 0.000005 AUR minimum
        }
    }

    /// Calculer les frais d'une transaction
    pub fn calculate_fee(&self, tx_size: usize, priority: Priority) -> u64 {
        let base = self.base_fee;
        let size_fee = tx_size as u64 * self.per_byte_fee;
        
        let total = base + size_fee;
        
        // Appliquer le multiplicateur de priorité
        let fee = match priority {
            Priority::Low => total,
            Priority::Medium => (total as f64 * 1.5) as u64,
            Priority::High => (total as f64 * 2.5) as u64,
        };

        // S'assurer que les frais sont au moins égaux au minimum relay
        fee.max(self.min_relay_fee)
    }

    /// Calculer les frais recommandés selon la congestion du réseau
    pub fn estimate_fees(&self, pending_tx_count: usize) -> FeeEstimate {
        // Calculer le multiplicateur basé sur la congestion
        let congestion_multiplier = if pending_tx_count > 10000 {
            5.0  // Très congestionné
        } else if pending_tx_count > 5000 {
            3.0  // Congestionné
        } else if pending_tx_count > 1000 {
            2.0  // Modéré
        } else {
            1.0  // Normal
        };

        let base = (self.base_fee as f64 * congestion_multiplier) as u64;

        FeeEstimate {
            low: base,
            medium: (base as f64 * 1.5) as u64,
            high: (base as f64 * 2.5) as u64,
            per_byte: (self.per_byte_fee as f64 * congestion_multiplier) as u64,
        }
    }

    /// Vérifier si les frais sont suffisants
    pub fn verify_fee(&self, tx_size: usize, paid_fee: u64) -> bool {
        let min_fee = self.calculate_fee(tx_size, Priority::Low);
        paid_fee >= min_fee
    }

    /// Calculer le montant en AUR (pour affichage)
    pub fn to_aur(satoshis: u64) -> f64 {
        satoshis as f64 / 100_000_000.0
    }
}

impl Default for TransactionFees {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_calculation() {
        let fees = TransactionFees::new();
        
        // Transaction de 250 bytes
        let fee_low = fees.calculate_fee(250, Priority::Low);
        let fee_medium = fees.calculate_fee(250, Priority::Medium);
        let fee_high = fees.calculate_fee(250, Priority::High);

        assert!(fee_medium > fee_low);
        assert!(fee_high > fee_medium);
        
        println!("Frais LOW: {} AUR", TransactionFees::to_aur(fee_low));
        println!("Frais MEDIUM: {} AUR", TransactionFees::to_aur(fee_medium));
        println!("Frais HIGH: {} AUR", TransactionFees::to_aur(fee_high));
    }

    #[test]
    fn test_congestion_multiplier() {
        let fees = TransactionFees::new();
        
        let normal = fees.estimate_fees(100);
        let congested = fees.estimate_fees(6000);
        let very_congested = fees.estimate_fees(15000);

        assert!(congested.low > normal.low);
        assert!(very_congested.low > congested.low);
    }

    #[test]
    fn test_min_relay_fee() {
        let fees = TransactionFees::new();
        
        // Transaction très petite (10 bytes)
        let fee = fees.calculate_fee(10, Priority::Low);
        
        // Doit être au moins égal au min_relay_fee
        assert!(fee >= fees.min_relay_fee);
    }
}
