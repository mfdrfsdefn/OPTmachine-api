use crate::AppState;
use crate::dto::pool_parser::{PoolAccountResponse, PoolParserRequest};
use crate::utils::serde_helpers::pubkey_from_str;
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::Account;
pub fn routes() -> Router<AppState> {
    Router::new().route("/pool/parser", post(pool_parser_service))
}
pub async fn pool_parser_service(
    State(state): State<AppState>,
    Json(req): Json<PoolParserRequest>,
) -> Result<Json<PoolAccountResponse>, axum::http::StatusCode> {
    let res = state
        .pool_parser_service
        .parse_pool_account(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(res))
}
