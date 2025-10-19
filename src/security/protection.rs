use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NetworkProtection {
    request_history: HashMap<String, Vec<u64>>,
    blacklist: Vec<String>,
    max_requests_per_minute: usize,
    max_connections_per_ip: usize,
}

impl NetworkProtection {
    pub fn new() -> Self {
        NetworkProtection {
            request_history: HashMap::new(),
            blacklist: Vec::new(),
            max_requests_per_minute: 60,
            max_connections_per_ip: 10,
        }
    }

    /// Vérifier si une requête est autorisée
    pub fn check_request(&mut self, ip: &str) -> bool {
        // Vérifier blacklist
        if self.blacklist.contains(&ip.to_string()) {
            log::warn!("🚫 Blocked request from blacklisted IP: {}", ip);
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Récupérer ou créer l'historique
        let history = self.request_history
            .entry(ip.to_string())
            .or_insert_with(Vec::new);

        // Nettoyer les vieilles entrées (> 1 minute)
        history.retain(|&timestamp| now - timestamp < 60);

        // Vérifier le nombre de requêtes
        if history.len() >= self.max_requests_per_minute {
            log::warn!(
                "⚠️  Rate limit exceeded for IP {}: {} requests/min",
                ip, history.len()
            );
            
            // Blacklist après 3 violations
            let violation_count = history.len() - self.max_requests_per_minute;
            if violation_count >= 3 {
                self.blacklist.push(ip.to_string());
                log::error!("🚨 IP {} added to blacklist", ip);
            }
            
            return false;
        }

        // Ajouter la requête actuelle
        history.push(now);
        true
    }

    /// Réinitialiser un IP (pour tests ou admin)
    pub fn whitelist(&mut self, ip: &str) {
        self.blacklist.retain(|i| i != ip);
        self.request_history.remove(ip);
        log::info!("✅ IP {} whitelisted", ip);
    }

    /// Obtenir les statistiques
    pub fn get_stats(&self) -> ProtectionStats {
        ProtectionStats {
            blacklisted_ips: self.blacklist.len(),
            monitored_ips: self.request_history.len(),
            total_requests: self.request_history.values()
                .map(|h| h.len())
                .sum(),
        }
    }
}

pub struct ProtectionStats {
    pub blacklisted_ips: usize,
    pub monitored_ips: usize,
    pub total_requests: usize,
}

impl ProtectionStats {
    pub fn log(&self) {
        log::info!("🛡️  Protection Stats:");
        log::info!("   Blacklisted IPs: {}", self.blacklisted_ips);
        log::info!("   Monitored IPs: {}", self.monitored_ips);
        log::info!("   Total requests: {}", self.total_requests);
    }
}
