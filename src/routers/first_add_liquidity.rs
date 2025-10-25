use crate::clients::first_add_liquidity;
use crate::AppState;
use crate::dto::first_add_liquidity::{FirstAddLiquidityRequest, FirstAddLiquidityResponse}; 
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/firstadd", post(first_add_liquidity))
}

pub async fn first_add_liquidity(
    State(state): State<AppState>,
    Json(req): Json<FirstAddLiquidityRequest>,
) -> Json<FirstAddLiquidityResponse> {
    let response = state
        .first_add_liquidity_service
        .build_first_add_liquidity_tx(req)
        .await
        .unwrap();
    Json(response)
}