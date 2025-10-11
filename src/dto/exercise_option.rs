use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::utils::serde_helpers::pubkey_from_str;

#[derive(Deserialize)]
pub struct ExerciseOptionRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub exerciser: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub option_mint: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub quote_mint: Pubkey,
    pub exercise_amount:u64,
}
#[derive(Serialize)]
pub struct ExerciseOptionResponse {
    pub unsigned_tx: String,
    pub exercise_amount:u64,
}