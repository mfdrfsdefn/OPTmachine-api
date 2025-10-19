use crate::utils::serde_helpers::pubkey_from_str;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
#[derive(Deserialize)]
pub struct ReclaimAssetRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub reclaimer: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub option_mint: Pubkey,
}
#[derive(Serialize)]
pub struct ReclaimAssetResponse {
    pub unsigned_tx: String,
}
#[derive(BorshDeserialize, BorshSerialize)]
pub struct OptionAccount {
    pub creator: Pubkey,
    pub underlying_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub option_mint: Pubkey,
    pub strike_price: u64,
    pub unix_expiration: i64,
    pub contract_size: u64,
    pub bump: u8,
}
