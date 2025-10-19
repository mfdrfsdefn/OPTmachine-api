use crate::AppState;
use crate::dto::reclaim_asset::{ReclaimAssetRequest, ReclaimAssetResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
pub fn routes() -> Router<AppState> {
    Router::new().route("/reclaim", post(reclaim_asset))
}

pub async fn reclaim_asset(
    State(state): State<AppState>,
    Json(req): Json<ReclaimAssetRequest>,
) -> Json<ReclaimAssetResponse> {
    let response = state
        .reclaim_asset_service
        .build_reclaim_asset_tx(req)
        .await
        .unwrap();

    Json(response)
}
