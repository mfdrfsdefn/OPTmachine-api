use std::{mem::swap, option};

use crate::state::AppState;
use axum::Router;
pub mod add_liquidity;
pub mod create_amm_pool;
pub mod create_option;
pub mod exercise_option;
pub mod first_add_liquidity;
pub mod health;
pub mod mint_option;
pub mod option_parser;
pub mod pool_parser;
pub mod reclaim_asset;
pub mod swap;
pub fn init_routes() -> Router<AppState> {
    Router::new()
        .merge(create_option::routes())
        .merge(mint_option::routes())
        .merge(exercise_option::routes())
        .merge(reclaim_asset::routes())
        .merge(create_amm_pool::routes())
        .merge(first_add_liquidity::routes())
        .merge(add_liquidity::routes())
        .merge(swap::routes())
        .merge(pool_parser::routes())
        .merge(option_parser::routes())
}
