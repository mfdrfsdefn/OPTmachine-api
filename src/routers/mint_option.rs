use axum::{Router, routing::post, extract::{State, Json}};
use crate::AppState;
use crate::dto::mint_option::{MintOptionRequest, MintOptionResponse};
pub fn routes() -> Router<AppState> {
    Router::new().route("/mint", post(mint_option))
}

pub async fn mint_option(
    State(state): State<AppState>,
    Json(req): Json<MintOptionRequest>,
) -> Json<MintOptionResponse> {
    let response = state
        .mint_option_service
        .build_mint_option_tx(req)
        .await
        .unwrap();

    Json(response)
}
