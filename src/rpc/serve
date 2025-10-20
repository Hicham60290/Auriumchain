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
    } else if path == "/chain_info" {
        get_chain_info(blockchain).await
    } else if path.starts_with("/blocks_from/") {
        let height_str = path.strip_prefix("/blocks_from/").unwrap_or("0");
        let from_height = height_str.parse().unwrap_or(0);
        get_blocks_from(blockchain, from_height).await
    } else {
        r#"{"error":"Not found"}"#.to_string()
    };
    
    let http_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );
    writer.write_all(http_response.as_bytes()).await?;
    
    Ok(())
}
