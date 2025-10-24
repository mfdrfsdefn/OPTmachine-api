use crate::utils::serde_helpers::pubkey_from_str;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct CreateAmmPoolRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub creator: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub mint_a: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub mint_b: Pubkey,
}
#[derive(Serialize)]
pub struct CreateAmmPoolResponse {
    pub unsigned_tx: String,
    pub pool_account: String,
}
