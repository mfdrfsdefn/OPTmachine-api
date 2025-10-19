use crate::AppState;
use crate::dto::exercise_option::{ExerciseOptionRequest, ExerciseOptionResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/exercise", post(exercise_option))
}

pub async fn exercise_option(
    State(state): State<AppState>,
    Json(req): Json<ExerciseOptionRequest>,
) -> Json<ExerciseOptionResponse> {
    let response = state
        .exercise_option_service
        .build_exercise_option_tx(req)
        .await
        .unwrap();
    Json(response)
}
