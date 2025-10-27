use crate::dto::airdrop;
use crate::AppState;
use crate::dto::airdrop::{AirdropRequest, AirdropResponse};
use crate::services::airdrop_service;
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/airdrop", post(airdrop))
}

pub async fn airdrop(
    State(state): State<AppState>,
    Json(req): Json<AirdropRequest>,
) -> Json<AirdropResponse> {
    let response = state
        .airdrop_service
        .build_airdrop_tx(req)
        .await
        .unwrap();
    Json(response)
}
