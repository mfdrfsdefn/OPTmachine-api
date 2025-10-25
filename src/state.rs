use crate::services::{
    create_option_service::CreateOptionService, exercise_option_service::ExerciseOptionService,
    mint_option_service::MintOptionService, reclaim_asset_service::ReclaimAssetService,
    create_amm_pool_service::CreateAmmPoolService,
    first_add_liquidity_service::FirstAddLiquidityService,
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::sync::Arc;
#[derive(Clone)]
pub struct AppState {
    pub create_option_service: Arc<CreateOptionService>,
    pub mint_option_service: Arc<MintOptionService>,
    pub exercise_option_service: Arc<ExerciseOptionService>,
    pub reclaim_asset_service: Arc<ReclaimAssetService>,
    pub create_amm_pool_service: Arc<CreateAmmPoolService>,
    pub first_add_liquidity_service: Arc<FirstAddLiquidityService>,
}
