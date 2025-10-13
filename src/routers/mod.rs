use axum::{Router};
use crate::{routers::exercise_option::exercise_option, state::AppState};
pub mod health;
pub mod create_option;
pub mod mint_option;
pub mod exercise_option;
pub fn init_routes() -> Router<AppState> {
    Router::new()
        .merge(create_option::routes()) 
        .merge(mint_option::routes())
        .merge(exercise_option::routes())   
}