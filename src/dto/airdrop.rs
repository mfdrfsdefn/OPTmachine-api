use crate::utils::serde_helpers::pubkey_from_str;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
#[derive(Deserialize)]
pub struct AirdropRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub user: Pubkey,
}
#[derive(Serialize)]
pub struct AirdropResponse {
    pub ok: bool,
    pub tx: String,
}
