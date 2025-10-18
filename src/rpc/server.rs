use axum::{
    routing::get,  // <-- Retirez "post"
    Router,
    Json,
    extract::State,
};
use std::sync::{Arc, Mutex};
use crate::blockchain::Blockchain;  // <-- Changez cette ligne
use serde_json::{json, Value};  // <-- Retirez Deserialize et Serialize

// ... le reste du code reste identique
pub async fn start_rpc_server(port: u16, blockchain: Arc<Mutex<Blockchain>>) {
    let app = Router::new()
        .route("/status", get(get_status))
        .route("/blocks", get(get_blocks))
        .route("/block/latest", get(get_latest_block))
        .route("/block/:index", get(get_block_by_index))
        .route("/balance/:address", get(get_balance))
        .route("/chain/validate", get(validate_chain))
        .with_state(blockchain);

    let addr = format!("0.0.0.0:{}", port);
    println!("📡 RPC Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_status(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    
    Json(json!({
        "status": "running",
        "block_height": chain.chain.len() - 1,
        "latest_hash": chain.get_latest_block().hash,
        "difficulty": chain.difficulty,
        "pending_transactions": chain.pending_transactions.len(),
        "is_valid": chain.is_valid(),
        "version": "1.0.0",
    }))
}

async fn get_blocks(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    Json(json!(chain.chain))
}

async fn get_latest_block(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    Json(json!(chain.get_latest_block()))
}

async fn get_block_by_index(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    axum::extract::Path(index): axum::extract::Path<usize>,
) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    
    if index < chain.chain.len() {
        Json(json!(chain.chain[index]))
    } else {
        Json(json!({
            "error": "Block not found"
        }))
    }
}

async fn get_balance(
    State(blockchain): State<Arc<Mutex<Blockchain>>>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    let balance = chain.get_balance(&address);
    
    Json(json!({
        "address": address,
        "balance": balance,
        "balance_aur": (balance as f64) / 100_000_000.0,
    }))
}

async fn validate_chain(State(blockchain): State<Arc<Mutex<Blockchain>>>) -> Json<Value> {
    let chain = blockchain.lock().unwrap();
    let is_valid = chain.is_valid();
    
    Json(json!({
        "is_valid": is_valid,
        "block_count": chain.chain.len(),
    }))
}
