use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

pub struct PeerManager {
    peers: RwLock<HashMap<SocketAddr, PeerConnection>>,
    max_peers: usize,
}

pub struct PeerConnection {
    pub addr: SocketAddr,
    pub stream: Option<TcpStream>,
    pub last_seen: u64,
    pub chain_length: u64,
}

impl PeerManager {
    pub fn new(max_peers: usize) -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
            max_peers,
        }
    }
    
    pub async fn add_peer(&self, addr: SocketAddr) -> bool {
        let mut peers = self.peers.write().await;
        
        if peers.len() >= self.max_peers {
            return false;
        }
        
        if !peers.contains_key(&addr) {
            let peer = PeerConnection {
                addr,
                stream: None,
                last_seen: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                chain_length: 0,
            };
            peers.insert(addr, peer);
            println!("Added peer: {}", addr);
            true
        } else {
            false
        }
    }
    
    pub async fn get_peers(&self) -> Vec<SocketAddr> {
        self.peers.read().await.keys().cloned().collect()
    }
    
    pub async fn update_peer_chain_length(&self, addr: SocketAddr, length: u64) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(&addr) {
            peer.chain_length = length;
        }
    }
    
    pub async fn get_best_peer(&self) -> Option<SocketAddr> {
        let peers = self.peers.read().await;
        peers.values()
            .max_by_key(|p| p.chain_length)
            .map(|p| p.addr)
    }
}

impl PeerManager {
    pub async fn get_all_peers(&self) -> Vec<std::net::SocketAddr> {
        // Return the actual list of connected peers
        self.peers.read().await.keys().cloned().collect()
    }
}
