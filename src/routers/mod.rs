use crate::{routers::exercise_option::exercise_option, state::AppState};
use axum::Router;
pub mod create_option;
pub mod exercise_option;
pub mod health;
pub mod mint_option;
pub mod reclaim_asset;
pub fn init_routes() -> Router<AppState> {
    Router::new()
        .merge(create_option::routes())
        .merge(mint_option::routes())
        .merge(exercise_option::routes())
}
