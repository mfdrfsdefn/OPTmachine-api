use crate::utils::serde_helpers::pubkey_from_str;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
#[derive(Deserialize)]
pub struct PoolParserRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub pool_account: Pubkey,
}
#[derive(Serialize)]
pub struct PoolAccountResponse {
    pub creator: String,
    pub mint_a: String,
    pub mint_b: String,
    pub amount_a: String,
    pub amount_b: String,
    pub k_human: String,
    pub unix_expiration: i64,
    pub expiration_human: String,
}
