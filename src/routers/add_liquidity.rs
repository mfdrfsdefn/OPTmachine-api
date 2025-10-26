use crate::AppState;
use crate::clients::first_add_liquidity;
use crate::dto::add_liquidity::{AddLiquidityRequest, AddLiquidityResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/add", post(add_liquidity))
}

pub async fn add_liquidity(
    State(state): State<AppState>,
    Json(req): Json<AddLiquidityRequest>,
) -> Json<AddLiquidityResponse> {
    let response = state
        .add_liquidity_service
        .build_add_liquidity_tx(req)
        .await
        .unwrap();
    Json(response)
}
