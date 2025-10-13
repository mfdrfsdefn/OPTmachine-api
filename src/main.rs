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
use crate::services::exercise_option_service::ExerciseOptionService;
use crate::{clients::exercise_option::ExerciseOptionArgs, services::create_option_service::CreateOptionService};
use crate::services::mint_option_service::MintOptionService;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let program_id = std::env::var("OPTMACHINE_PROGRAM_ID")
        .expect("OPTMACHINE_PROGRAM_ID must be set")
        .parse::<Pubkey>()
        .expect("Invalid OPTMACHINE_PROGRAM_ID");

    let rpc_url = std::env::var("SOLANA_RPC_PRIMARY")
        .expect("SOLANA_RPC_PRIMARY must be set");

    println!("Primary RPC = {:?}", std::env::var("SOLANA_RPC_PRIMARY"));

    let create_option_service = Arc::new(CreateOptionService::new(&rpc_url, program_id));
    let mint_option_service = Arc::new(MintOptionService::new(&rpc_url, program_id));
    let exercise_option_service =Arc::new(ExerciseOptionService::new(&rpc_url, program_id));
    let state = AppState { create_option_service,mint_option_service,exercise_option_service };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(routers::init_routes())
        .route("/", get(|| async { "Hello, OptMachine API 🚀" }))
        .with_state(state.clone())
        .layer(cors);   

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
