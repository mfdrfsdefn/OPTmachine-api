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
use crate::services::swap_service;
use crate::{
    clients::exercise_option::ExerciseOptionArgs,
    services::create_option_service::CreateOptionService,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
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
        let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");
    let rpc_url = std::env::var("SOLANA_RPC_PRIMARY").expect("SOLANA_RPC_PRIMARY must be set");
    let keypair_str = std::env::var("AIRDROP_KEYPAIR").expect("AIRDROP_WALLET_KEYPAIR must be set");
    println!("Primary RPC = {:?}", rpc_url);
    let keypair_vec: Vec<u8> = serde_json::from_str(&keypair_str)
        .expect("Invalid AIRDROP_KEYPAIR format — must be JSON array");
    let airdrop_keypair: [u8; 64] = keypair_vec
        .try_into()
        .expect("AIRDROP_KEYPAIR must be an array of 64 bytes");
    let keypair_32: [u8; 32] = airdrop_keypair[0..32]
        .try_into()
        .expect("Failed to extract first 32 bytes for Keypair");
    let airdrop_wallet = Arc::new(Keypair::new_from_array(keypair_32));
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
        services::add_liquidity_service::AddLiquidityService::new(&rpc_url, amm_program_id),
    );
    let swap_service = Arc::new(services::swap_service::SwapService::new(
        &rpc_url,
        amm_program_id,
    ));
    let pool_parser_service = Arc::new(services::pool_parser_service::PoolParserService::new(
        &rpc_url,
    ));
    let option_parser_service = Arc::new(
        services::option_parser_service::OptionParserService::new(&rpc_url),
    );
    let airdrop_service = Arc::new(services::airdrop_service::AirdropService::new(
        &rpc_url,
        airdrop_wallet.clone(),
    ));
    let state = AppState {
        create_option_service,
        mint_option_service,
        exercise_option_service,
        reclaim_asset_service,
        create_amm_pool_service,
        first_add_liquidity_service,
        add_liquidity_service,
        swap_service,
        pool_parser_service,
        option_parser_service,
        airdrop_service,
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

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
