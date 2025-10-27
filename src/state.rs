use crate::services::{
    add_liquidity_service::AddLiquidityService, create_amm_pool_service::CreateAmmPoolService,
    create_option_service::CreateOptionService, exercise_option_service::ExerciseOptionService,
    first_add_liquidity_service::FirstAddLiquidityService, mint_option_service::MintOptionService,
    pool_parser_service::PoolParserService, reclaim_asset_service::ReclaimAssetService,
    swap_service::SwapService,
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
    pub add_liquidity_service: Arc<AddLiquidityService>,
    pub swap_service: Arc<SwapService>,
    pub pool_parser_service: Arc<PoolParserService>,
}
