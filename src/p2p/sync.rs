use crate::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SyncMessage {
    RequestChainInfo,
    ChainInfo { 
        height: u64, 
        latest_hash: String 
    },
    RequestBlocks { 
        from_height: u64 
    },
    SendBlocks { 
        blocks: Vec<crate::blockchain::Block> 
    },
    NewBlock { 
        block: crate::blockchain::Block 
    },
}

#[derive(Debug, Deserialize)]
struct PeerStatus {
    status: String,
    block_height: u64,
    latest_hash: String,
}

#[derive(Debug, Deserialize)]
struct PeerChainInfo {
    height: u64,
    latest_hash: String,
    difficulty: u32,
}

pub struct SyncManager {
    blockchain: Arc<RwLock<Blockchain>>,
    peer_manager: Arc<crate::p2p::PeerManager>,
    client: reqwest::Client,
}

impl SyncManager {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>, peer_manager: Arc<crate::p2p::PeerManager>) -> Self {
        Self {
            blockchain,
            peer_manager,
            client: reqwest::Client::new(),
        }
    }

    pub async fn broadcast_new_block(&self, block: crate::blockchain::Block) -> Result<(), Box<dyn std::error::Error>> {
        let peers = self.peer_manager.get_all_peers().await;
        
        for peer_addr in peers {
            match self.send_block_to_peer(peer_addr, &block).await {
                Ok(_) => println!("✅ Block sent to peer: {}", peer_addr),
                Err(e) => println!("❌ Failed to send block to peer {}: {}", peer_addr, e),
            }
        }
        
        Ok(())
    }
    
    pub async fn sync_with_peer(&self, peer_addr: std::net::SocketAddr) -> Result<bool, Box<dyn std::error::Error>> {
        println!("🔄 Starting sync with peer: {}", peer_addr);
        
        let our_height = {
            let chain = self.blockchain.read().await;
            chain.get_chain_length()
        };
        
        // Vraie requête HTTP pour obtenir la hauteur du peer
        let peer_height = match self.get_peer_chain_height(peer_addr).await {
            Ok(height) => height,
            Err(e) => {
                println!("❌ Failed to get peer height from {}: {}", peer_addr, e);
                return Ok(false);
            }
        };
        
        println!("📊 Heights - Us: {}, Peer {}: {}", our_height, peer_addr, peer_height);
        
        if peer_height > our_height {
            println!("⬇️ Peer {} has longer chain ({} vs {}), downloading blocks...", peer_addr, peer_height, our_height);
            
            // Vraie requête HTTP pour télécharger les blocs
            let new_blocks = match self.download_blocks_from_peer(peer_addr, our_height).await {
                Ok(blocks) => blocks,
                Err(e) => {
                    println!("❌ Failed to download blocks from {}: {}", peer_addr, e);
                    return Ok(false);
                }
            };
            
            if new_blocks.is_empty() {
                println!("⚠️ No blocks received from peer");
                return Ok(false);
            }
            
            // Appliquer les nouveaux blocs
            let mut chain = self.blockchain.write().await;
            let mut applied_blocks = 0;

            println!("📥 Attempting to apply {} downloaded blocks...", new_blocks.len());

            for (i, block) in new_blocks.iter().enumerate() {
                let current_chain_len = chain.chain.len();
                let latest_block_info = if let Some(latest) = chain.get_latest_block() {
                    let hash_preview = if latest.hash.len() > 16 { &latest.hash[..16] } else { &latest.hash };
                    format!("index={}, hash={}", latest.index, hash_preview)
                } else {
                    "empty chain".to_string()
                };

                let prev_hash_preview = if block.previous_hash.len() > 16 { &block.previous_hash[..16] } else { &block.previous_hash };
                println!("🔍 Validating block {}/{}: index={}, prev_hash={}, current_chain_len={}, latest_block=[{}]",
                    i+1, new_blocks.len(), block.index, prev_hash_preview, current_chain_len, latest_block_info);

                if chain.validate_new_block(block) {
                    chain.chain.push(block.clone());
                    applied_blocks += 1;
                    println!("✓ Block {} accepted (chain now has {} blocks)", block.index, chain.chain.len());
                } else {
                    println!("❌ Rejected invalid block {} from peer (validation failed)", block.index);
                    let block_hash_preview = if block.hash.len() > 32 { &block.hash[..32] } else { &block.hash };
                    let calc_hash = block.calculate_hash();
                    let calc_hash_preview = if calc_hash.len() > 32 { &calc_hash[..32] } else { &calc_hash };
                    println!("   Block hash: {}", block_hash_preview);
                    println!("   Calculated: {}", calc_hash_preview);
                    break;
                }
            }

            if applied_blocks > 0 {
                println!("✅ Successfully synchronized {} new blocks from {} (chain now: {} blocks)",
                    applied_blocks, peer_addr, chain.chain.len());
                return Ok(true);
            }
        } else if peer_height < our_height {
            println!("⬆️ We have longer chain ({} vs {}), peer should sync from us", our_height, peer_height);
        } else {
            println!("✅ Chains are in sync ({} blocks)", our_height);
        }
        
        Ok(false)
    }
    
    // Vraie requête HTTP pour obtenir la hauteur de chaîne du peer
    async fn get_peer_chain_height(&self, peer_addr: std::net::SocketAddr) -> Result<usize, Box<dyn std::error::Error>> {
        let rpc_port = self.get_rpc_port_for_peer(peer_addr);
        let url = format!("http://{}:{}/status", peer_addr.ip(), rpc_port);
        
        println!("🌐 Requesting status from: {}", url);
        
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            self.client.get(&url).send()
        ).await??;
        
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        
        let status: PeerStatus = response.json().await?;
        
        println!("📡 Peer {} status: {} blocks", peer_addr, status.block_height);
        
        Ok(status.block_height as usize)
    }
    
    // Vraie requête HTTP pour télécharger les blocs depuis un peer
    async fn download_blocks_from_peer(&self, peer_addr: std::net::SocketAddr, from_height: usize) -> Result<Vec<crate::blockchain::Block>, Box<dyn std::error::Error>> {
        let rpc_port = self.get_rpc_port_for_peer(peer_addr);
        let url = format!("http://{}:{}/blocks_from/{}", peer_addr.ip(), rpc_port, from_height);
        
        println!("⬇️ Downloading blocks from: {}", url);
        
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.client.get(&url).send()
        ).await??;
        
        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }
        
        let blocks: Vec<crate::blockchain::Block> = response.json().await?;
        
        println!("📦 Downloaded {} blocks from peer {}", blocks.len(), peer_addr);
        
        Ok(blocks)
    }
    
    // Envoyer un nouveau bloc à un peer (pour le broadcasting)
    async fn send_block_to_peer(&self, peer_addr: std::net::SocketAddr, block: &crate::blockchain::Block) -> Result<(), Box<dyn std::error::Error>> {
        let rpc_port = self.get_rpc_port_for_peer(peer_addr);
        let url = format!("http://{}:{}/new_block", peer_addr.ip(), rpc_port);
        
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            self.client.post(&url).json(block).send()
        ).await;
        
        match response {
            Ok(Ok(resp)) if resp.status().is_success() => {
                println!("📤 Block sent to peer: {}", peer_addr);
                Ok(())
            },
            Ok(Ok(resp)) => Err(format!("HTTP error: {}", resp.status()).into()),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err("Request timeout".into()),
        }
    }
    
    // Obtenir le port RPC basé sur le port P2P
    fn get_rpc_port_for_peer(&self, peer_addr: std::net::SocketAddr) -> u16 {
        match peer_addr.port() {
            3001 => 8001,
            3002 => 8002,
            3003 => 8003,
            _ => 8001,
        }
    }
}

pub struct BlockchainSync {
    blockchain: Arc<RwLock<Blockchain>>,
    peers: Vec<String>,
}

impl BlockchainSync {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>) -> Self {
        Self {
            blockchain,
            peers: Vec::new(),
        }
    }
}