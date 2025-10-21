use crate::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::RwLock;
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

pub async fn get_status(blockchain: Arc<RwLock<Blockchain>>) -> String {
    let chain = blockchain.read().await;
    let latest_block = chain.chain.last();
    
    format!(
        r#"{{"status":"running","version":"1.0.0","block_height":{},"latest_hash":"{}","difficulty":4,"is_valid":true,"pending_transactions":0}}"#,
        if chain.chain.is_empty() { 0 } else { chain.chain.len() - 1 },
        latest_block.map(|b| b.hash.as_str()).unwrap_or("none")
    )
}

async fn get_all_blocks(
    blockchain: Arc<RwLock<Blockchain>>,
) -> String {
    let chain = blockchain.read().await;
    
    match serde_json::to_string(&chain.chain) {
        Ok(json) => json,
        Err(_) => r#"{"error":"Serialization failed"}"#.to_string(),
    }
}

async fn handle_balance_request(
    blockchain: Arc<RwLock<Blockchain>>,
    path: &str,
) -> String {
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

async fn get_chain_info(
    blockchain: Arc<RwLock<Blockchain>>,
) -> String {
    let chain = blockchain.read().await;
    let height = chain.get_chain_length() as u64;
    let latest_hash = chain.get_latest_block()
        .map(|b| b.hash.clone())
        .unwrap_or_default();
    
    format!(
        r#"{{"height":{},"latest_hash":"{}","difficulty":{}}}"#,
        height, latest_hash, chain.get_difficulty()
    )
}

async fn get_blocks_from(
    blockchain: Arc<RwLock<Blockchain>>,
    from_height: u64,
) -> String {
    let chain = blockchain.read().await;
    let blocks: Vec<_> = chain.chain.iter()
        .skip(from_height as usize)
        .collect();
    
    match serde_json::to_string(&blocks) {
        Ok(json) => json,
        Err(_) => r#"{"error":"Serialization failed"}"#.to_string(),
    }
}

async fn handle_new_block(
    blockchain: Arc<RwLock<Blockchain>>,
    body: &str,
) -> String {
    match serde_json::from_str::<crate::blockchain::Block>(body) {
        Ok(block) => {
            let mut chain = blockchain.write().await;
            if chain.validate_new_block(&block) {
                chain.chain.push(block);
                if let Err(e) = chain.save_to_file("/tmp/auriumchain.json") {
                    eprintln!("Failed to save blockchain: {}", e);
                }
                r#"{"status":"block_accepted"}"#.to_string()
            } else {
                r#"{"error":"invalid_block"}"#.to_string()
            }
        },
        Err(_) => r#"{"error":"invalid_json"}"#.to_string(),
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    blockchain: Arc<RwLock<Blockchain>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; 8192];
    let (mut reader, mut writer) = stream.into_split();
    let n = reader.read(&mut buffer).await?;
    let request = String::from_utf8_lossy(&buffer[..n]);
    
    let (method, path, body) = if let Some(first_line) = request.lines().next() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let method = parts.get(0).unwrap_or(&"GET");
        let path = parts.get(1).unwrap_or(&"/status");
        
        let body = if let Some(body_start) = request.find("\r\n\r\n") {
            &request[body_start + 4..]
        } else {
            ""
        };
        
        (*method, *path, body)
    } else {
        ("GET", "/status", "")
    };
    
    let response = match (method, path) {
        ("GET", "/status") => get_status(blockchain).await,
        ("GET", "/blocks") => get_all_blocks(blockchain).await,
        ("GET", "/chain_info") => get_chain_info(blockchain).await,
        ("GET", path) if path.starts_with("/balance/") => handle_balance_request(blockchain, path).await,
        ("GET", path) if path.starts_with("/blocks_from/") => {
            let height_str = path.strip_prefix("/blocks_from/").unwrap_or("0");
            let from_height = height_str.parse().unwrap_or(0);
            get_blocks_from(blockchain, from_height).await
        },
        ("POST", "/new_block") => handle_new_block(blockchain, body).await,
        _ => r#"{"error":"Not found"}"#.to_string(),
    };
    
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );
    writer.write_all(http_response.as_bytes()).await?;
    
    Ok(())
}