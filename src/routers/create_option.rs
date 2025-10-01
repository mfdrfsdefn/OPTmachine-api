use axum::{Router, routing::{get,post}, extract::{State, Json}};
use crate::AppState;
use crate::dto::create_option::{CreateOptionRequest, CreateOptionResponse};
pub fn routes() -> Router<AppState> {
    Router::new().route("/create", post(create_option))
}

pub async fn create_option(
    State(state): State<AppState>,
    Json(req): Json<CreateOptionRequest>,
) -> Json<CreateOptionResponse> {
    let response = state.create_option_service.build_create_option_tx(req).await.unwrap();
    Json(response)
}