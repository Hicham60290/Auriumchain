use std::collections::HashMap;
use std::net::IpAddr;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
struct ConnectionStats {
    connection_count: usize,
    last_reset: Instant,
    blocks_received: Vec<Instant>,
    messages_received: Vec<Instant>,
    banned_until: Option<Instant>,
}

impl ConnectionStats {
    fn new() -> Self {
        ConnectionStats {
            connection_count: 0,
            last_reset: Instant::now(),
            blocks_received: Vec::new(),
            messages_received: Vec::new(),
            banned_until: None,
        }
    }

    fn is_banned(&self) -> bool {
        if let Some(banned_until) = self.banned_until {
            Instant::now() < banned_until
        } else {
            false
        }
    }

    fn reset_if_needed(&mut self, window: Duration) {
        let now = Instant::now();
        if now.duration_since(self.last_reset) > window {
            self.blocks_received.clear();
            self.messages_received.clear();
            self.last_reset = now;
        }
    }

    fn cleanup_old_entries(&mut self, window: Duration) {
        let cutoff = Instant::now() - window;
        self.blocks_received.retain(|&time| time > cutoff);
        self.messages_received.retain(|&time| time > cutoff);
    }
}

pub struct RateLimiter {
    stats: Arc<Mutex<HashMap<IpAddr, ConnectionStats>>>,

    // Limits
    max_connections_per_ip: usize,
    max_blocks_per_minute: usize,
    max_messages_per_minute: usize,
    max_message_size_bytes: usize,

    // Ban settings
    ban_duration: Duration,
    violations_before_ban: usize,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            stats: Arc::new(Mutex::new(HashMap::new())),

            // Conservative limits for security
            max_connections_per_ip: 3,
            max_blocks_per_minute: 60,      // 1 block/second max
            max_messages_per_minute: 300,   // 5 messages/second max
            max_message_size_bytes: 10_000_000, // 10 MB max message

            // Ban for 15 minutes after 5 violations
            ban_duration: Duration::from_secs(15 * 60),
            violations_before_ban: 5,
        }
    }

    /// Check if an IP is allowed to connect
    pub fn allow_connection(&self, ip: IpAddr) -> Result<()> {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(ip).or_insert_with(ConnectionStats::new);

        // Check if banned
        if stats.is_banned() {
            return Err(anyhow!("IP {} is temporarily banned", ip));
        }

        // Check connection limit
        if stats.connection_count >= self.max_connections_per_ip {
            log::warn!("Rate limit: IP {} exceeded max connections ({})", ip, self.max_connections_per_ip);
            return Err(anyhow!("Too many connections from IP {}", ip));
        }

        stats.connection_count += 1;
        Ok(())
    }

    /// Record disconnection
    pub fn record_disconnection(&self, ip: IpAddr) {
        let mut stats_map = self.stats.lock().unwrap();
        if let Some(stats) = stats_map.get_mut(&ip) {
            if stats.connection_count > 0 {
                stats.connection_count -= 1;
            }
        }
    }

    /// Check if a block can be accepted from this IP
    pub fn allow_block(&self, ip: IpAddr) -> Result<()> {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(ip).or_insert_with(ConnectionStats::new);

        // Check if banned
        if stats.is_banned() {
            return Err(anyhow!("IP {} is temporarily banned", ip));
        }

        // Cleanup old entries
        stats.cleanup_old_entries(Duration::from_secs(60));

        // Check block rate limit
        if stats.blocks_received.len() >= self.max_blocks_per_minute {
            log::warn!("Rate limit: IP {} exceeded max blocks per minute ({})",
                       ip, self.max_blocks_per_minute);

            self.record_violation(ip);
            return Err(anyhow!("Block rate limit exceeded for IP {}", ip));
        }

        stats.blocks_received.push(Instant::now());
        Ok(())
    }

    /// Check if a message can be accepted
    pub fn allow_message(&self, ip: IpAddr, message_size: usize) -> Result<()> {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(ip).or_insert_with(ConnectionStats::new);

        // Check if banned
        if stats.is_banned() {
            return Err(anyhow!("IP {} is temporarily banned", ip));
        }

        // Check message size
        if message_size > self.max_message_size_bytes {
            log::warn!("Rate limit: IP {} sent oversized message ({} bytes)", ip, message_size);
            self.record_violation(ip);
            return Err(anyhow!("Message size exceeds limit"));
        }

        // Cleanup old entries
        stats.cleanup_old_entries(Duration::from_secs(60));

        // Check message rate limit
        if stats.messages_received.len() >= self.max_messages_per_minute {
            log::warn!("Rate limit: IP {} exceeded max messages per minute ({})",
                       ip, self.max_messages_per_minute);

            self.record_violation(ip);
            return Err(anyhow!("Message rate limit exceeded for IP {}", ip));
        }

        stats.messages_received.push(Instant::now());
        Ok(())
    }

    /// Record a violation and potentially ban the IP
    fn record_violation(&self, ip: IpAddr) {
        // This is a simplified violation tracking
        // In production, you'd want more sophisticated tracking

        let mut stats_map = self.stats.lock().unwrap();
        if let Some(stats) = stats_map.get_mut(&ip) {
            // For now, ban immediately on any violation
            // TODO: Implement proper violation counting
            stats.banned_until = Some(Instant::now() + self.ban_duration);
            log::error!("⚠️  IP {} has been banned for {:?}", ip, self.ban_duration);
        }
    }

    /// Manually ban an IP
    pub fn ban_ip(&self, ip: IpAddr, duration: Duration) {
        let mut stats_map = self.stats.lock().unwrap();
        let stats = stats_map.entry(ip).or_insert_with(ConnectionStats::new);
        stats.banned_until = Some(Instant::now() + duration);
        log::error!("🔒 IP {} manually banned for {:?}", ip, duration);
    }

    /// Unban an IP
    pub fn unban_ip(&self, ip: IpAddr) {
        let mut stats_map = self.stats.lock().unwrap();
        if let Some(stats) = stats_map.get_mut(&ip) {
            stats.banned_until = None;
            log::info!("🔓 IP {} has been unbanned", ip);
        }
    }

    /// Get statistics for an IP
    pub fn get_stats(&self, ip: IpAddr) -> Option<String> {
        let stats_map = self.stats.lock().unwrap();
        stats_map.get(&ip).map(|stats| {
            format!(
                "IP: {}\nConnections: {}\nBlocks/min: {}\nMessages/min: {}\nBanned: {}",
                ip,
                stats.connection_count,
                stats.blocks_received.len(),
                stats.messages_received.len(),
                stats.is_banned()
            )
        })
    }

    /// Get list of all banned IPs
    pub fn get_banned_ips(&self) -> Vec<IpAddr> {
        let stats_map = self.stats.lock().unwrap();
        stats_map.iter()
            .filter(|(_, stats)| stats.is_banned())
            .map(|(ip, _)| *ip)
            .collect()
    }

    /// Cleanup old entries periodically
    pub fn cleanup(&self) {
        let mut stats_map = self.stats.lock().unwrap();
        let now = Instant::now();

        // Remove entries that haven't been active for 1 hour
        stats_map.retain(|_, stats| {
            // Keep if recently active or currently banned
            now.duration_since(stats.last_reset) < Duration::from_secs(3600)
                || stats.is_banned()
        });
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_connection_limit() {
        let limiter = RateLimiter::new();
        let ip = IpAddr::from_str("192.168.1.1").unwrap();

        // Allow first 3 connections
        assert!(limiter.allow_connection(ip).is_ok());
        assert!(limiter.allow_connection(ip).is_ok());
        assert!(limiter.allow_connection(ip).is_ok());

        // 4th connection should fail
        assert!(limiter.allow_connection(ip).is_err());

        // After disconnect, should work again
        limiter.record_disconnection(ip);
        assert!(limiter.allow_connection(ip).is_ok());
    }

    #[test]
    fn test_message_size_limit() {
        let limiter = RateLimiter::new();
        let ip = IpAddr::from_str("192.168.1.2").unwrap();

        // Small message OK
        assert!(limiter.allow_message(ip, 1000).is_ok());

        // Oversized message should fail
        assert!(limiter.allow_message(ip, 20_000_000).is_err());
    }

    #[test]
    fn test_ban_unban() {
        let limiter = RateLimiter::new();
        let ip = IpAddr::from_str("192.168.1.3").unwrap();

        // Ban IP
        limiter.ban_ip(ip, Duration::from_secs(10));

        // Should reject connection
        assert!(limiter.allow_connection(ip).is_err());

        // Unban
        limiter.unban_ip(ip);

        // Should allow connection now
        assert!(limiter.allow_connection(ip).is_ok());
    }
}
