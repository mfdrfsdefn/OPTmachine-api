use crate::services::{
    create_option_service::CreateOptionService, exercise_option_service::ExerciseOptionService,
    mint_option_service::MintOptionService, reclaim_asset_service::ReclaimAssetService,
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    pub create_option_service: Arc<CreateOptionService>,
    pub mint_option_service: Arc<MintOptionService>,
    pub exercise_option_service: Arc<ExerciseOptionService>,
    pub reclaim_asset_service: Arc<ReclaimAssetService>,
}
