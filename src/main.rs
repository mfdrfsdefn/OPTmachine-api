use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::sync::Arc;

mod routers;
mod state;
mod services;
use state::AppState;
mod dto;
mod utils;
mod clients;

use solana_sdk::pubkey::Pubkey;
use crate::services::create_option_service::CreateOptionService;

// 👇 新增：CORS 支持
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    // 读取环境变量
    let program_id = std::env::var("OPTMACHINE_PROGRAM_ID")
        .expect("OPTMACHINE_PROGRAM_ID must be set")
        .parse::<Pubkey>()
        .expect("Invalid OPTMACHINE_PROGRAM_ID");

    let rpc_url = std::env::var("SOLANA_RPC_PRIMARY")
        .expect("SOLANA_RPC_PRIMARY must be set");

    println!("Primary RPC = {:?}", std::env::var("SOLANA_RPC_PRIMARY"));

    // 初始化服务和状态
    let create_option_service = Arc::new(CreateOptionService::new(&rpc_url, program_id));
    let state = AppState { create_option_service };

    // 👇 定义 CORS 策略（允许任意来源、方法、Header）
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 构建 Axum 应用
    let app = Router::new()
        .merge(routers::init_routes())
        .route("/", get(|| async { "Hello, OptMachine API 🚀" }))
        .with_state(state.clone())
        .layer(cors);   // 👈 加上 CORS

    // 启动服务
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
