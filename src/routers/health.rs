use crate::state::AppState;
use axum::{Router, routing::get};
pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}
