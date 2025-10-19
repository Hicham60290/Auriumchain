use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Statistiques énergétiques d'un bloc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEnergyStats {
    pub block_index: u64,
    pub hash_attempts: u64,        // Nombre de hashes calculés
    pub mining_duration_secs: f64, // Temps de minage en secondes
    pub estimated_watts: f64,      // Puissance estimée (W)
    pub estimated_wh: f64,         // Énergie consommée (Wh)
    pub difficulty: u32,
}

/// Calculateur d'énergie
pub struct EnergyCalculator {
    pub watts_per_mhash: f64,  // Watts par million de hashes
    pub base_power: f64,       // Puissance de base (W)
}

impl EnergyCalculator {
    pub fn new() -> Self {
        EnergyCalculator {
            // Estimation pour CPU moderne (i5/i7)
            watts_per_mhash: 0.5,  // 0.5W par MHash/s
            base_power: 50.0,      // 50W de base
        }
    }

    /// Calculer l'énergie consommée pour miner un bloc
    pub fn calculate_block_energy(
        &self,
        hash_attempts: u64,
        duration_secs: f64,
    ) -> BlockEnergyStats {
        // Calculer le hashrate (hashes/seconde)
        let hashrate = hash_attempts as f64 / duration_secs;
        let hashrate_mh = hashrate / 1_000_000.0; // Convertir en MH/s
        
        // Puissance estimée
        let mining_power = self.base_power + (hashrate_mh * self.watts_per_mhash);
        
        // Énergie consommée (Wh)
        let energy_wh = mining_power * (duration_secs / 3600.0);

        BlockEnergyStats {
            block_index: 0,
            hash_attempts,
            mining_duration_secs: duration_secs,
            estimated_watts: mining_power,
            estimated_wh: energy_wh,
            difficulty: 0,
        }
    }

    /// Estimer l'énergie pour une difficulté donnée
    pub fn estimate_energy_for_difficulty(&self, difficulty: u32) -> f64 {
        // Nombre approximatif de hashes nécessaires
        let expected_hashes = 2u64.pow(difficulty);
        
        // Temps estimé (en supposant 1 MH/s)
        let estimated_secs = expected_hashes as f64 / 1_000_000.0;
        
        // Énergie (Wh)
        self.base_power * (estimated_secs / 3600.0)
    }
}

impl Default for EnergyCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracker d'énergie global
pub struct EnergyTracker {
    pub total_blocks_mined: u64,
    pub total_energy_wh: f64,
    pub total_hash_attempts: u64,
    pub total_mining_time_secs: f64,
    pub calculator: EnergyCalculator,
}

impl EnergyTracker {
    pub fn new() -> Self {
        EnergyTracker {
            total_blocks_mined: 0,
            total_energy_wh: 0.0,
            total_hash_attempts: 0,
            total_mining_time_secs: 0.0,
            calculator: EnergyCalculator::new(),
        }
    }

    /// Enregistrer un bloc miné
    pub fn record_block(&mut self, stats: BlockEnergyStats) {
        self.total_blocks_mined += 1;
        self.total_energy_wh += stats.estimated_wh;
        self.total_hash_attempts += stats.hash_attempts;
        self.total_mining_time_secs += stats.mining_duration_secs;
    }

    /// Obtenir l'énergie moyenne par bloc
    pub fn average_energy_per_block(&self) -> f64 {
        if self.total_blocks_mined == 0 {
            0.0
        } else {
            self.total_energy_wh / self.total_blocks_mined as f64
        }
    }

    /// Obtenir l'énergie totale en kWh
    pub fn total_energy_kwh(&self) -> f64 {
        self.total_energy_wh / 1000.0
    }

    /// Comparer avec d'autres blockchains
    pub fn compare_with_others(&self) -> EnergyComparison {
        let energy_per_tx = if self.total_blocks_mined > 0 {
            // Supposons 10 TX par bloc en moyenne
            self.total_energy_wh / (self.total_blocks_mined as f64 * 10.0)
        } else {
            0.0
        };

        EnergyComparison {
            auriumchain_wh_per_tx: energy_per_tx,
            bitcoin_wh_per_tx: 150_000.0,      // ~150 kWh
            ethereum_pow_wh_per_tx: 60_000.0,   // ~60 kWh
            ethereum_pos_wh_per_tx: 10.0,       // ~0.01 kWh
        }
    }

    /// Afficher les statistiques
    pub fn print_stats(&self) {
        println!("\n╔════════════════════════════════════════════════╗");
        println!("║     STATISTIQUES ÉNERGÉTIQUES AURIUMCHAIN      ║");
        println!("╠════════════════════════════════════════════════╣");
        println!("║                                                ║");
        println!("║  Blocs minés : {:<32} ║", self.total_blocks_mined);
        println!("║  Énergie totale : {:<25.2} kWh ║", self.total_energy_kwh());
        println!("║  Énergie/bloc : {:<27.4} Wh ║", self.average_energy_per_block());
        println!("║  Temps total : {:<26.1} heures ║", self.total_mining_time_secs / 3600.0);
        println!("║  Hashes totaux : {:<30} ║", self.total_hash_attempts);
        println!("║                                                ║");
        
        let comparison = self.compare_with_others();
        println!("║  🌍 COMPARAISON (par transaction) :           ║");
        println!("║  ├─ AuriumChain : {:<26.4} Wh ║", comparison.auriumchain_wh_per_tx);
        println!("║  ├─ Bitcoin : {:<30.0} Wh ║", comparison.bitcoin_wh_per_tx);
        println!("║  ├─ Ethereum PoW : {:<25.0} Wh ║", comparison.ethereum_pow_wh_per_tx);
        println!("║  └─ Ethereum PoS : {:<26.1} Wh ║", comparison.ethereum_pos_wh_per_tx);
        println!("║                                                ║");
        
        let reduction_vs_btc = (1.0 - comparison.auriumchain_wh_per_tx / comparison.bitcoin_wh_per_tx) * 100.0;
        println!("║  ✅ Réduction vs Bitcoin : {:<17.1}% ║", reduction_vs_btc);
        println!("║                                                ║");
        println!("╚════════════════════════════════════════════════╝\n");
    }
}

impl Default for EnergyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyComparison {
    pub auriumchain_wh_per_tx: f64,
    pub bitcoin_wh_per_tx: f64,
    pub ethereum_pow_wh_per_tx: f64,
    pub ethereum_pos_wh_per_tx: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_calculation() {
        let calc = EnergyCalculator::new();
        
        // 1 million de hashes en 1 seconde
        let stats = calc.calculate_block_energy(1_000_000, 1.0);
        
        assert!(stats.estimated_watts > 0.0);
        assert!(stats.estimated_wh > 0.0);
        
        println!("Puissance: {} W", stats.estimated_watts);
        println!("Énergie: {} Wh", stats.estimated_wh);
    }

    #[test]
    fn test_tracker() {
        let mut tracker = EnergyTracker::new();
        
        // Enregistrer 3 blocs
        for i in 0..3 {
            let stats = BlockEnergyStats {
                block_index: i,
                hash_attempts: 100_000,
                mining_duration_secs: 2.0,
                estimated_watts: 55.0,
                estimated_wh: 0.031,
                difficulty: 4,
            };
            tracker.record_block(stats);
        }
        
        assert_eq!(tracker.total_blocks_mined, 3);
        assert!(tracker.total_energy_wh > 0.0);
        
        tracker.print_stats();
    }

    #[test]
    fn test_comparison() {
        let mut tracker = EnergyTracker::new();
        tracker.total_blocks_mined = 100;
        tracker.total_energy_wh = 500.0; // 500 Wh pour 100 blocs
        
        let comparison = tracker.compare_with_others();
        
        // AuriumChain devrait être beaucoup plus efficace que Bitcoin
        assert!(comparison.auriumchain_wh_per_tx < comparison.bitcoin_wh_per_tx / 1000.0);
    }
}
