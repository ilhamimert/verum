// VERUM Web Arayüzü — blok gezgini + cüzdan paneli
// Çalıştır: cargo run --bin verum-web  →  http://127.0.0.1:3000

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use verum_core::blockchain::Blockchain;
use verum_core::transaction::{Transaction, TransactionType};
use verum_core::wallet::Wallet;

const FAUCET_AMOUNT: f64 = 100.0;
const FOUNDATION_ADDRESS: &str = "VERUM_FOUNDATION";

#[derive(Clone)]
struct AppState {
    blockchain: Arc<Mutex<Blockchain>>,
    wallets: Arc<Mutex<HashMap<String, Wallet>>>,
}

#[derive(Deserialize)]
struct SendRequest {
    from: String,
    to: String,
    amount: f64,
}

#[derive(Deserialize)]
struct FaucetRequest {
    address: String,
}

#[derive(Serialize)]
struct ApiResponse {
    success: bool,
    data: Value,
    error: Option<String>,
}

impl ApiResponse {
    fn ok(data: Value) -> Json<Self> {
        Json(ApiResponse { success: true, data, error: None })
    }

    fn err(message: &str) -> (StatusCode, Json<Self>) {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: Value::Null,
                error: Some(message.to_string()),
            }),
        )
    }
}

type ApiResult = Result<Json<ApiResponse>, (StatusCode, Json<ApiResponse>)>;

#[tokio::main]
async fn main() {
    let state = AppState {
        blockchain: Arc::new(Mutex::new(Blockchain::new())),
        wallets: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/api/status", get(status))
        .route("/api/chain", get(chain))
        .route("/api/mempool", get(mempool))
        .route("/api/wallet", post(create_wallet))
        .route("/api/balance/{address}", get(balance))
        .route("/api/faucet", post(faucet))
        .route("/api/send", post(send))
        .route("/api/mine", post(mine))
        .with_state(state);

    let addr = "127.0.0.1:3000";
    println!("🌐 VERUM Web Arayüzü hazır → http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("3000 portu açılamadı");
    axum::serve(listener, app).await.expect("sunucu hatası");
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

async fn status(State(state): State<AppState>) -> Json<ApiResponse> {
    let bc = state.blockchain.lock().unwrap();
    ApiResponse::ok(json!({
        "block_count": bc.block_count(),
        "pending_count": bc.pending_transactions.len(),
        "chain_valid": bc.is_valid(),
    }))
}

async fn chain(State(state): State<AppState>) -> Json<ApiResponse> {
    let bc = state.blockchain.lock().unwrap();
    ApiResponse::ok(json!(bc.chain))
}

async fn mempool(State(state): State<AppState>) -> Json<ApiResponse> {
    let bc = state.blockchain.lock().unwrap();
    ApiResponse::ok(json!(bc.pending_transactions))
}

async fn create_wallet(State(state): State<AppState>) -> Json<ApiResponse> {
    let wallet = Wallet::new();
    let info = wallet.info.clone();
    state
        .wallets
        .lock()
        .unwrap()
        .insert(info.address.clone(), wallet);
    ApiResponse::ok(json!(info))
}

async fn balance(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Json<ApiResponse> {
    let bc = state.blockchain.lock().unwrap();
    ApiResponse::ok(json!({
        "address": address,
        "balance": bc.get_balance(&address),
    }))
}

async fn faucet(
    State(state): State<AppState>,
    Json(req): Json<FaucetRequest>,
) -> ApiResult {
    if req.address.trim().is_empty() {
        return Err(ApiResponse::err("adres boş olamaz"));
    }

    let mut bc = state.blockchain.lock().unwrap();
    let tx = Transaction::new(
        FOUNDATION_ADDRESS,
        &req.address,
        FAUCET_AMOUNT,
        TransactionType::Transfer,
    );

    if !bc.add_transaction(tx) {
        return Err(ApiResponse::err("musluk işlemi reddedildi"));
    }

    Ok(ApiResponse::ok(json!({
        "message": format!("{FAUCET_AMOUNT} VRM mempool'a eklendi — blok kazılınca bakiyene geçer")
    })))
}

async fn send(
    State(state): State<AppState>,
    Json(req): Json<SendRequest>,
) -> ApiResult {
    if req.amount <= 0.0 {
        return Err(ApiResponse::err("miktar 0'dan büyük olmalı"));
    }

    let wallets = state.wallets.lock().unwrap();
    let wallet = wallets
        .get(&req.from)
        .ok_or_else(|| ApiResponse::err("gönderen cüzdan bu sunucuda kayıtlı değil"))?;

    let mut bc = state.blockchain.lock().unwrap();
    let available = bc.get_balance(&req.from);
    let fee = req.amount * 0.001;
    if available < req.amount + fee {
        return Err(ApiResponse::err(&format!(
            "yetersiz bakiye: {available:.3} VRM var, {:.3} VRM gerekli (ücret dahil)",
            req.amount + fee
        )));
    }

    let tx = wallet.create_transaction(&req.to, req.amount);
    let tx_id = tx.id.clone();

    if !bc.add_transaction(tx) {
        return Err(ApiResponse::err("işlem reddedildi"));
    }

    Ok(ApiResponse::ok(json!({ "tx_id": tx_id })))
}

async fn mine(State(state): State<AppState>) -> ApiResult {
    let mut bc = state.blockchain.lock().unwrap();
    match bc.mine_pending_transactions() {
        Some(block) => {
            let block = block.clone();
            Ok(ApiResponse::ok(json!(block)))
        }
        None => Err(ApiResponse::err(
            "kazılacak işlem yok ya da PoAI konsensüsü bloğu reddetti",
        )),
    }
}
