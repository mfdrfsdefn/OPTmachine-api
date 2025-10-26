use crate::utils::serde_helpers::pubkey_from_str;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct SwapRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub provider: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub pool: Pubkey,
    pub amount_in: u64,
    pub a_to_b: bool,
}
#[derive(Serialize)]
pub struct SwapResponse {
    pub unsigned_tx: String,
}
