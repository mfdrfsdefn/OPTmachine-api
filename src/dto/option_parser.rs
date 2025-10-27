use crate::utils::serde_helpers::pubkey_from_str;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Deserialize)]
pub struct OptionParserRequest {
    #[serde(deserialize_with = "pubkey_from_str")]
    pub option_mint: Pubkey,
}

#[derive(Serialize)]
pub struct OptionParserResponse {
    pub creator: String,
    pub underlying_mint: String,
    pub quote_mint: String,
    pub option_mint: String,
    pub strike_price: String,
    pub unix_expiration: u64,
    pub expiration_human: String,
    pub contract_size: String,
    pub contract_size_human: String,
    pub strike_price_human: String,
    pub total_supply: String,
}
