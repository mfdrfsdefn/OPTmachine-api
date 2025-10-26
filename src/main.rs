use axum::{Router, routing::get, serve};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

mod routers;
mod services;
mod state;
use state::AppState;
mod clients;
mod dto;
mod utils;

use crate::services::exercise_option_service::ExerciseOptionService;
use crate::services::mint_option_service::MintOptionService;
use crate::{
    clients::exercise_option::ExerciseOptionArgs,
    services::create_option_service::CreateOptionService,
};
use solana_sdk::pubkey::Pubkey;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let program_id = std::env::var("OPTMACHINE_PROGRAM_ID")
        .expect("OPTMACHINE_PROGRAM_ID must be set")
        .parse::<Pubkey>()
        .expect("Invalid OPTMACHINE_PROGRAM_ID");
    let amm_program_id = std::env::var("OPTMACHINE_AMM_PROGRAM_ID")
        .expect("OPTMACHINE_AMM_PROGRAM_ID must be set")
        .parse::<Pubkey>()
        .expect("Invalid OPTMACHINE_AMM_PROGRAM_ID");

    let rpc_url = std::env::var("SOLANA_RPC_PRIMARY").expect("SOLANA_RPC_PRIMARY must be set");

    println!("Primary RPC = {:?}", std::env::var("SOLANA_RPC_PRIMARY"));

    let create_option_service = Arc::new(CreateOptionService::new(&rpc_url, program_id));
    let mint_option_service = Arc::new(MintOptionService::new(&rpc_url, program_id));
    let exercise_option_service = Arc::new(ExerciseOptionService::new(&rpc_url, program_id));
    let create_amm_pool_service = Arc::new(
        services::create_amm_pool_service::CreateAmmPoolService::new(&rpc_url, amm_program_id),
    );
    let reclaim_asset_service = Arc::new(
        services::reclaim_asset_service::ReclaimAssetService::new(&rpc_url, program_id),
    );
    let first_add_liquidity_service = Arc::new(
        services::first_add_liquidity_service::FirstAddLiquidityService::new(
            &rpc_url,
            amm_program_id,
        ),
    );
        let add_liquidity_service = Arc::new(
            services::add_liquidity_service::AddLiquidityService::new(
                &rpc_url,
                amm_program_id,
            ),
        );
    let state = AppState {
        create_option_service,
        mint_option_service,
        exercise_option_service,
        reclaim_asset_service,
        create_amm_pool_service,
        first_add_liquidity_service,
        add_liquidity_service,
    };

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
