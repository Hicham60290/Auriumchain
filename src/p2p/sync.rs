use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::blockchain::{Blockchain, Block};
use crate::p2p::{P2PMessage, PeerManager};

pub struct SyncManager {
    blockchain: Arc<RwLock<Blockchain>>,
    peer_manager: Arc<PeerManager>,
}

impl SyncManager {
    pub fn new(
        blockchain: Arc<RwLock<Blockchain>>,
        peer_manager: Arc<PeerManager>
    ) -> Self {
        Self {
            blockchain,
            peer_manager,
        }
    }
    
    pub async fn sync_with_peer(&self, peer_addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(peer_addr).await?;
        
        // Envoyer handshake
        let chain = self.blockchain.read().await;
        let my_length = chain.chain.len() as u64;
        let genesis_hash = chain.chain.first()
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "none".to_string());
        drop(chain);
        
        let handshake = P2PMessage::Handshake {
            version: "1.0".to_string(),
            chain_length: my_length,
            peer_id: format!("node-{}", 12345),
            genesis_hash,
        };
        
        self.send_message(&mut stream, &handshake).await?;
        
        // Lire la réponse
        if let Ok(response) = self.read_message(&mut stream).await {
            match response {
                P2PMessage::Handshake { chain_length, .. } => {
                    if chain_length > my_length {
                        println!("Peer has longer chain ({} vs {}), requesting sync", chain_length, my_length);
                        self.request_chain_sync(&mut stream, my_length, chain_length).await?;
                    }
                }
                _ => {}
            }
        }
        
        Ok(())
    }
    
    async fn request_chain_sync(
        &self,
        stream: &mut TcpStream,
        from: u64,
        to: u64
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = P2PMessage::RequestBlocks {
            from_height: from,
            to_height: to,
        };
        
        self.send_message(stream, &request).await?;
        
        if let Ok(P2PMessage::SendBlocks { blocks }) = self.read_message(stream).await {
            self.process_received_blocks(blocks).await?;
        }
        
        Ok(())
    }
    
    async fn process_received_blocks(&self, blocks: Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
        let mut chain = self.blockchain.write().await;
        
        for block in blocks {
            // Validation basique
            if let Some(last_block) = chain.chain.last() {
                if block.index == last_block.index + 1 && block.previous_hash == last_block.hash {
                    chain.chain.push(block.clone());
                    println!("Added block #{} to chain", block.index);
                }
            } else if block.index == 0 {
                // Genesis block
                chain.chain.push(block.clone());
                println!("Added Genesis block to chain");
            }
        }
        
        Ok(())
    }
    
    async fn send_message(
        &self,
        stream: &mut TcpStream,
        message: &P2PMessage
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = message.serialize();
        let length = data.len() as u32;
        
        stream.write_all(&length.to_be_bytes()).await?;
        stream.write_all(&data).await?;
        
        Ok(())
    }
    
    async fn read_message(
        &self,
        stream: &mut TcpStream
    ) -> Result<P2PMessage, Box<dyn std::error::Error>> {
        let mut length_buf = [0u8; 4];
        stream.read_exact(&mut length_buf).await?;
        let length = u32::from_be_bytes(length_buf) as usize;
        
        let mut data = vec![0u8; length];
        stream.read_exact(&mut data).await?;
        
        P2PMessage::deserialize(&data)
    }
}
