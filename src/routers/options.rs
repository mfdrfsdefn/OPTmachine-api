use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::state::AppState;
#[derive(Serialize)]
struct OptionInfo {
    id: u64,
    name: String,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/{id}", get(get_option))
}

async fn get_option() -> Json<OptionInfo> {
    Json(OptionInfo {
        id: 1,
        name: "SOL Call Option".to_string(),
    })
}
