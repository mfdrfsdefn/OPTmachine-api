use crate::AppState;
use crate::clients::create_amm_pool;
use crate::dto::create_amm_pool::{CreateAmmPoolRequest, CreateAmmPoolResponse};
use crate::services::create_amm_pool_service;
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/create_amm", post(create_amm_pool))
}

pub async fn create_amm_pool(
    State(state): State<AppState>,
    Json(req): Json<CreateAmmPoolRequest>,
) -> Json<CreateAmmPoolResponse> {
    let response = state
        .create_amm_pool_service
        .build_create_amm_pool_tx(req)
        .await
        .unwrap();
    Json(response)
}
