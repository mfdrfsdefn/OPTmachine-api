use crate::AppState;
use crate::dto::option_parser::{OptionParserRequest, OptionParserResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/option/parser", post(option_parser_handler))
}

pub async fn option_parser_handler(
    State(state): State<AppState>,
    Json(req): Json<OptionParserRequest>,
) -> Result<Json<OptionParserResponse>, axum::http::StatusCode> {
    let res = state
        .option_parser_service
        .parse_option_account(req)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(res))
}
