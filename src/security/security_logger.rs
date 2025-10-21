use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    // Connection events
    PeerConnected,
    PeerDisconnected,
    ConnectionRefused,
    ConnectionBanned,

    // Block events
    BlockReceived,
    BlockAccepted,
    BlockRejected,
    InvalidBlock,

    // Transaction events
    TransactionReceived,
    TransactionRejected,
    InvalidSignature,
    DoubleSpendDetected,

    // Security violations
    RateLimitExceeded,
    OversizedMessage,
    InvalidProofOfWork,
    ChainReorgDetected,
    SuspiciousActivity,

    // System events
    NodeStarted,
    NodeStopped,
    ConfigChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: i64,
    pub event_type: SecurityEventType,
    pub peer_ip: Option<String>,
    pub details: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

pub struct SecurityLogger {
    log_file: Arc<Mutex<File>>,
    log_path: PathBuf,
    max_file_size: u64,
}

impl SecurityLogger {
    pub fn new<P: AsRef<Path>>(log_dir: P) -> Result<Self, std::io::Error> {
        // Create log directory if it doesn't exist
        std::fs::create_dir_all(&log_dir)?;

        let log_path = log_dir.as_ref().join("security.log");

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        Ok(SecurityLogger {
            log_file: Arc::new(Mutex::new(log_file)),
            log_path,
            max_file_size: 100_000_000, // 100 MB
        })
    }

    /// Log a security event
    pub fn log_event(&self, event: SecurityEvent) {
        // Console log based on severity
        match event.severity {
            Severity::Info => log::info!("[SECURITY] {} - {}", self.event_type_str(&event.event_type), event.details),
            Severity::Warning => log::warn!("[SECURITY] {} - {}", self.event_type_str(&event.event_type), event.details),
            Severity::Error => log::error!("[SECURITY] {} - {}", self.event_type_str(&event.event_type), event.details),
            Severity::Critical => log::error!("🚨 [CRITICAL SECURITY] {} - {}", self.event_type_str(&event.event_type), event.details),
        }

        // Write to file
        if let Ok(mut file) = self.log_file.lock() {
            let log_line = format!(
                "{} | {:?} | {:?} | {} | {}\n",
                chrono::DateTime::<Utc>::from_timestamp(event.timestamp, 0)
                    .map(|dt| dt.to_rfc3339())
                    .unwrap_or_else(|| "UNKNOWN".to_string()),
                event.severity,
                event.event_type,
                event.peer_ip.as_ref().unwrap_or(&"N/A".to_string()),
                event.details
            );

            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
        }

        // Check if rotation needed
        self.rotate_if_needed();
    }

    fn event_type_str(&self, event_type: &SecurityEventType) -> &str {
        match event_type {
            SecurityEventType::PeerConnected => "PEER_CONNECTED",
            SecurityEventType::PeerDisconnected => "PEER_DISCONNECTED",
            SecurityEventType::ConnectionRefused => "CONNECTION_REFUSED",
            SecurityEventType::ConnectionBanned => "CONNECTION_BANNED",
            SecurityEventType::BlockReceived => "BLOCK_RECEIVED",
            SecurityEventType::BlockAccepted => "BLOCK_ACCEPTED",
            SecurityEventType::BlockRejected => "BLOCK_REJECTED",
            SecurityEventType::InvalidBlock => "INVALID_BLOCK",
            SecurityEventType::TransactionReceived => "TX_RECEIVED",
            SecurityEventType::TransactionRejected => "TX_REJECTED",
            SecurityEventType::InvalidSignature => "INVALID_SIGNATURE",
            SecurityEventType::DoubleSpendDetected => "DOUBLE_SPEND",
            SecurityEventType::RateLimitExceeded => "RATE_LIMIT",
            SecurityEventType::OversizedMessage => "OVERSIZED_MESSAGE",
            SecurityEventType::InvalidProofOfWork => "INVALID_POW",
            SecurityEventType::ChainReorgDetected => "CHAIN_REORG",
            SecurityEventType::SuspiciousActivity => "SUSPICIOUS",
            SecurityEventType::NodeStarted => "NODE_STARTED",
            SecurityEventType::NodeStopped => "NODE_STOPPED",
            SecurityEventType::ConfigChanged => "CONFIG_CHANGED",
        }
    }

    /// Rotate log file if it exceeds max size
    fn rotate_if_needed(&self) {
        if let Ok(metadata) = std::fs::metadata(&self.log_path) {
            if metadata.len() > self.max_file_size {
                self.rotate_log();
            }
        }
    }

    fn rotate_log(&self) {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let archive_path = self.log_path.with_extension(format!("log.{}", timestamp));

        // Rename current log
        if std::fs::rename(&self.log_path, &archive_path).is_ok() {
            log::info!("Security log rotated to: {:?}", archive_path);

            // Create new log file
            if let Ok(new_file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_path) {
                if let Ok(mut file) = self.log_file.lock() {
                    *file = new_file;
                }
            }

            // Cleanup old logs (keep last 10)
            self.cleanup_old_logs();
        }
    }

    fn cleanup_old_logs(&self) {
        if let Some(parent) = self.log_path.parent() {
            if let Ok(entries) = std::fs::read_dir(parent) {
                let mut log_files: Vec<_> = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().extension().map_or(false, |ext| {
                            let ext_str = ext.to_string_lossy();
                            ext_str.starts_with("log.") || ext_str == "log"
                        })
                    })
                    .collect();

                log_files.sort_by_key(|e| e.metadata().ok().and_then(|m| m.modified().ok()));

                // Keep only last 10 files
                if log_files.len() > 10 {
                    for file in log_files.iter().take(log_files.len() - 10) {
                        let _ = std::fs::remove_file(file.path());
                        log::info!("Removed old security log: {:?}", file.path());
                    }
                }
            }
        }
    }

    // Convenience methods for common events

    pub fn log_peer_connected(&self, peer_ip: String) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::PeerConnected,
            peer_ip: Some(peer_ip),
            details: "Peer connection established".to_string(),
            severity: Severity::Info,
        });
    }

    pub fn log_peer_disconnected(&self, peer_ip: String, reason: &str) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::PeerDisconnected,
            peer_ip: Some(peer_ip),
            details: format!("Peer disconnected: {}", reason),
            severity: Severity::Info,
        });
    }

    pub fn log_block_rejected(&self, peer_ip: Option<String>, reason: &str, block_index: u64) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::BlockRejected,
            peer_ip,
            details: format!("Block {} rejected: {}", block_index, reason),
            severity: Severity::Warning,
        });
    }

    pub fn log_invalid_signature(&self, peer_ip: Option<String>, tx_id: &str) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::InvalidSignature,
            peer_ip,
            details: format!("Invalid signature in transaction {}", tx_id),
            severity: Severity::Error,
        });
    }

    pub fn log_rate_limit_exceeded(&self, peer_ip: String, reason: &str) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::RateLimitExceeded,
            peer_ip: Some(peer_ip),
            details: format!("Rate limit exceeded: {}", reason),
            severity: Severity::Warning,
        });
    }

    pub fn log_double_spend(&self, peer_ip: Option<String>, details: &str) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::DoubleSpendDetected,
            peer_ip,
            details: details.to_string(),
            severity: Severity::Critical,
        });
    }

    pub fn log_chain_reorg(&self, old_height: u64, new_height: u64, common_ancestor: u64) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::ChainReorgDetected,
            peer_ip: None,
            details: format!(
                "Chain reorganization detected: {} -> {} (common ancestor: {})",
                old_height, new_height, common_ancestor
            ),
            severity: Severity::Warning,
        });
    }

    pub fn log_node_started(&self) {
        self.log_event(SecurityEvent {
            timestamp: Utc::now().timestamp(),
            event_type: SecurityEventType::NodeStarted,
            peer_ip: None,
            details: "AuriumChain node started".to_string(),
            severity: Severity::Info,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_security_logger_creation() {
        let dir = tempdir().unwrap();
        let logger = SecurityLogger::new(dir.path()).unwrap();

        logger.log_peer_connected("192.168.1.1".to_string());

        // Check log file was created
        assert!(dir.path().join("security.log").exists());
    }
}
