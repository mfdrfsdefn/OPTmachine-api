use axum::{Router, routing::post, extract::{State, Json}};
use crate::state::AppState;
pub mod health;
pub mod options;
pub mod create_option;
use crate::routers::create_option::create_option;
use crate::dto::create_option::*;
pub fn init_routes() -> Router<AppState> {
    Router::new().route("/create", post(create_option))
}
async fn create_option_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateOptionRequest>,
) -> Json<CreateOptionResponse> {
    let response = state
        .create_option_service
        .build_create_option_tx(req)   
        .await
        .expect("failed to build option tx");
    Json(response)
}
