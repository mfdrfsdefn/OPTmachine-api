use axum::{Router};
use crate::state::AppState;
pub mod health;
pub mod create_option;
pub mod mint_option;
pub fn init_routes() -> Router<AppState> {
    Router::new()
        .merge(create_option::routes()) 
        .merge(mint_option::routes())   
}