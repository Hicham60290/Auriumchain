use crate::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn get_status(blockchain: Arc<RwLock<Blockchain>>) -> String {
    let chain = blockchain.read().await;
    let latest_block = chain.chain.last();
    
    format!(
        r#"{{"status":"running","version":"1.0.0","block_height":{},"latest_hash":"{}","difficulty":4,"is_valid":true,"pending_transactions":0}}"#,
        if chain.chain.is_empty() { 0 } else { chain.chain.len() - 1 },
        latest_block.map(|b| b.hash.as_str()).unwrap_or("none")
    )
}

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_rpc_server(
    blockchain: Arc<RwLock<Blockchain>>, 
    port: u16
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("RPC Server listening on http://0.0.0.0:{}", port);
    
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let chain = blockchain.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, chain).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }
}




async fn handle_balance_request(
    blockchain: Arc<RwLock<Blockchain>>,
    path: &str,
) -> String {
    // Extraire l'adresse du path /balance/ADDRESS
    if let Some(address) = path.strip_prefix("/balance/") {
        let chain = blockchain.read().await;
        let balance = chain.get_balance(address);
        
        format!(
            r#"{{"address":"{}","balance":{},"currency":"AUR"}}"#,
            address, balance
        )
    } else {
        r#"{"error":"Invalid balance request"}"#.to_string()
    }
}
async fn handle_connection(
    stream: tokio::net::TcpStream,
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; 1024];
    let (mut reader, mut writer) = stream.into_split();
    let n = reader.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    
    // Extraire le path de la requête HTTP
    let path = if let Some(first_line) = request.lines().next() {
        if let Some(path_start) = first_line.find(' ') {
            if let Some(path_end) = first_line[path_start + 1..].find(' ') {
                &first_line[path_start + 1..path_start + 1 + path_end]
            } else {
                "/status"
            }
        } else {
            "/status"
        }
    } else {
        "/status"
    };
    
    let response = if path == "/status" {
        get_status(blockchain).await
    } else if path.starts_with("/balance/") {
        handle_balance_request(blockchain, path).await
    } else {
        r#"{"error":"Not found"}"#.to_string()
    };
    
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );
    writer.write_all(http_response.as_bytes()).await?;
    
    Ok(())
}
