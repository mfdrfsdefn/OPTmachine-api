use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use crate::utils::serde_helpers::pubkey_from_str;
use borsh::{BorshDeserialize,BorshSerialize};
#[derive(Deserialize)]
pub struct MintOptionRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub minter: Pubkey,
    #[serde(deserialize_with = "pubkey_from_str")]
    pub option_mint: Pubkey,
    pub mint_amount: u64,
}
#[derive(Serialize)]
pub struct MintOptionResponse {
    pub unsigned_tx: String,
    pub mint_amount: u64,
}
#[derive(BorshDeserialize,BorshSerialize)]
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