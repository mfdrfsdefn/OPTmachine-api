use crate::AppState;
use crate::dto::swap::{SwapRequest, SwapResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/swap", post(swap))
}

pub async fn swap(
    State(state): State<AppState>,
    Json(req): Json<SwapRequest>,
) -> Json<SwapResponse> {
    let response = state
        .swap_service
        .build_swap_tx(req)
        .await
        .unwrap();
    Json(response)
}
