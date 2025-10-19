use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::blockchain::Blockchain;
use crate::p2p::{P2PMessage, PeerManager, NetworkSecurity};

pub struct P2PServer {
    blockchain: Arc<RwLock<Blockchain>>,
    peer_manager: Arc<PeerManager>,
    security: Arc<NetworkSecurity>,
    bind_addr: SocketAddr,
}

impl P2PServer {
    pub fn new(
        blockchain: Arc<RwLock<Blockchain>>,
        peer_manager: Arc<PeerManager>,
        security: Arc<NetworkSecurity>,
        bind_addr: SocketAddr,
    ) -> Self {
        Self {
            blockchain,
            peer_manager,
            security,
            bind_addr,
        }
    }
    
    pub async fn start(&self) -> Result<(), anyhow::Error> {
        let listener = TcpListener::bind(self.bind_addr).await?;
        println!("P2P Server (TLS) listening on {}", self.bind_addr);
        
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    println!("New TLS P2P connection from: {}", peer_addr);
                    
                    let blockchain = self.blockchain.clone();
                    let peer_manager = self.peer_manager.clone();
                    let security = self.security.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(
                            stream, 
                            peer_addr, 
                            blockchain, 
                            peer_manager,
                            security
                        ).await {
                            eprintln!("TLS P2P connection error {}: {}", peer_addr, e);
                        }
                    });
                },
                Err(e) => eprintln!("Accept error: {}", e),
            }
        }
    }
    
    async fn handle_connection(
        stream: TcpStream,
        peer_addr: SocketAddr,
        blockchain: Arc<RwLock<Blockchain>>,
        peer_manager: Arc<PeerManager>,
        security: Arc<NetworkSecurity>,
    ) -> Result<(), anyhow::Error> {
        // Upgrade to TLS
        let tls_stream = security.tls_acceptor.accept(stream).await?;
        let (mut reader, mut writer) = tokio::io::split(tls_stream);
        
        peer_manager.add_peer(peer_addr).await;
        
        // Handshake sécurisé
        let chain = blockchain.read().await;
        let my_length = chain.chain.len() as u64;
        let genesis_hash = chain.chain.first()
            .map(|b| b.hash.clone())
            .unwrap_or_else(|| "none".to_string());
        drop(chain);
        
        let handshake = P2PMessage::Handshake {
            version: "1.0".to_string(),
            chain_length: my_length,
            peer_id: format!("tls-server-{}", peer_addr.port()),
            genesis_hash,
        };
        
        Self::send_message(&mut writer, &handshake).await?;
        println!("TLS P2P handshake sent to: {}", peer_addr);
        
        Ok(())
    }
    
    async fn send_message(
        writer: &mut tokio::io::WriteHalf<tokio_rustls::server::TlsStream<TcpStream>>,
        message: &P2PMessage,
    ) -> Result<(), anyhow::Error> {
        let data = message.serialize();
        let length = data.len() as u32;
        
        writer.write_all(&length.to_be_bytes()).await?;
        writer.write_all(&data).await?;
        
        Ok(())
    }
}
