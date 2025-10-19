use crate::blockchain::{Block, Transaction};
use chrono::Utc;

pub struct SecurityMonitor {
    alerts: Vec<SecurityAlert>,
    max_alerts: usize,
}

#[derive(Debug, Clone)]
pub struct SecurityAlert {
    pub timestamp: i64,
    pub level: AlertLevel,
    pub message: String,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

impl SecurityMonitor {
    pub fn new() -> Self {
        SecurityMonitor {
            alerts: Vec::new(),
            max_alerts: 1000,
        }
    }

    pub fn check_block(&mut self, block: &Block) {
        // 1. Vérifier la taille du bloc
        if let Ok(serialized) = bincode::serialize(block) {
            let size = serialized.len();
            if size > 3_000_000 {
                self.add_alert(
                    AlertLevel::Warning,
                    "Large block detected",
                    format!("Block {} size: {} bytes", block.index, size),
                );
            }
        }

        // 2. Vérifier le nombre de transactions
        if block.transactions.len() > 5000 {
            self.add_alert(
                AlertLevel::Warning,
                "High transaction count",
                format!("Block {} has {} transactions", block.index, block.transactions.len()),
            );
        }

        // 3. Vérifier le temps de minage (anormalement rapide = suspect)
        if block.nonce < 1000 {
            self.add_alert(
                AlertLevel::Info,
                "Block mined quickly",
                format!("Block {} mined with low nonce: {}", block.index, block.nonce),
            );
        }
    }

    pub fn check_transaction(&mut self, tx: &Transaction) {
        // 1. Transaction avec montant énorme
        let total_output: u64 = tx.outputs.iter().map(|o| o.value).sum();
        if total_output > 100_000_00000000 {
            self.add_alert(
                AlertLevel::Warning,
                "Large transaction detected",
                format!("TX {} amount: {} AUR", tx.id, total_output as f64 / 100_000_000.0),
            );
        }

        // 2. Transaction avec trop d'outputs
        if tx.outputs.len() > 1000 {
            self.add_alert(
                AlertLevel::Warning,
                "Transaction with many outputs",
                format!("TX {} has {} outputs", tx.id, tx.outputs.len()),
            );
        }

        // 3. Transaction sans inputs (sauf coinbase)
        if tx.inputs.is_empty() && !tx.is_coinbase() {
            self.add_alert(
                AlertLevel::Critical,
                "Invalid transaction: no inputs",
                format!("TX {} has no inputs but is not coinbase", tx.id),
            );
        }
    }

    pub fn detect_51_attack(&mut self, old_height: u64, new_height: u64) {
        if old_height > new_height + 6 {
            self.add_alert(
                AlertLevel::Critical,
                "Potential 51% attack detected",
                format!("Chain reorganization detected: {} -> {}", old_height, new_height),
            );
        }
    }

    fn add_alert(&mut self, level: AlertLevel, message: &str, details: String) {
    let alert = SecurityAlert {
        timestamp: Utc::now().timestamp(),
        level: level.clone(),
        message: message.to_string(),
        details: details.clone(), // Clone ici
    };

    // Log immédiatement
    match level {
        AlertLevel::Info => log::info!("ℹ️  {}: {}", message, &details),
        AlertLevel::Warning => log::warn!("⚠️  {}: {}", message, &details),
        AlertLevel::Critical => log::error!("🚨 CRITICAL: {}: {}", message, &details),
    }

    // Stocker
    self.alerts.push(alert);

    // Limiter la taille
    if self.alerts.len() > self.max_alerts {
        self.alerts.remove(0);
    }
}

    pub fn get_recent_alerts(&self, count: usize) -> Vec<SecurityAlert> {
        let start = if self.alerts.len() > count {
            self.alerts.len() - count
        } else {
            0
        };
        self.alerts[start..].to_vec()
    }

    pub fn get_critical_alerts(&self) -> Vec<SecurityAlert> {
        self.alerts
            .iter()
            .filter(|a| a.level == AlertLevel::Critical)
            .cloned()
            .collect()
    }

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }
}
