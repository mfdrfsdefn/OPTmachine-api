use crate::utils::serde_helpers::pubkey_from_str;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
#[derive(Deserialize)]
pub struct AddLiquidityRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub provider: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub pool: Pubkey,
    pub deposit_a: u64,
}
#[derive(Serialize)]
pub struct AddLiquidityResponse {
    pub unsigned_tx: String,
}
#[derive(BorshDeserialize, BorshSerialize)]

pub struct PoolAccount {
    pub creator: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub k: u128,
    pub unix_expiration: i64,
    pub bump: u8,
}
