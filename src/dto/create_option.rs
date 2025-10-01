use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::utils::serde_helpers::pubkey_from_str;

#[derive(Deserialize)]
pub struct CreateOptionRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub creator: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub underlying_mint: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub quote_mint: Pubkey,
    pub strike_price: u64,
    pub unix_expiration: i64,
    pub contract_size: u64,
}
#[derive(Serialize)]
pub struct CreateOptionResponse {
    pub unsigned_tx: String,
    pub option_token_mint: String,
}