use crate::dto::option_parser::{OptionParserRequest, OptionParserResponse};
use axum::http::StatusCode;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::Serialize;
use sha2::{Digest, Sha256};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
use std::sync::Arc;
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct OptionAccount {
    pub creator: [u8; 32],
    pub underlying_mint: [u8; 32],
    pub quote_mint: [u8; 32],
    pub option_mint: [u8; 32],
    pub strike_price: u64,
    pub unix_expiration: u64,
    pub contract_size: u64,
    pub bump: u8,
}

#[derive(Clone)]
pub struct OptionParserService {
    pub rpc: Arc<RpcClient>,
}

impl OptionParserService {
    pub fn new(rpc_url: &str) -> Self {
        let rpc = Arc::new(RpcClient::new(rpc_url.to_string()));
        Self { rpc }
    }

    pub async fn parse_option_account(
        &self,
        req: OptionParserRequest,
    ) -> Result<OptionParserResponse, StatusCode> {
        let option_mint_pubkey = req.option_mint;

        let mint_account = self
            .rpc
            .get_account(&option_mint_pubkey)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let mint_data = mint_account.data;
        if mint_data.len() < 82 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let mint = Mint::unpack(&mint_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let total_supply = mint.supply.to_string();

        let mint_authority_raw = &mint_data[4..36];
        let mint_authority = Pubkey::new_from_array(mint_authority_raw.try_into().unwrap());

        let account_info = self
            .rpc
            .get_account(&mint_authority)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let data = &account_info.data;
        let mut hasher = Sha256::new();
        hasher.update("account:OptionAccount");
        let discriminator = &hasher.finalize()[..8];

        if data.len() < 8 || &data[..8] != discriminator {
            return Err(StatusCode::BAD_REQUEST);
        }

        let option = OptionAccount::try_from_slice(&data[8..])
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let underlying_mint_pk = Pubkey::new_from_array(option.underlying_mint);
        let quote_mint_pk = Pubkey::new_from_array(option.quote_mint);

        let underlying_decimals = self.get_mint_decimals(&underlying_mint_pk).await?;
        let quote_decimals = self.get_mint_decimals(&quote_mint_pk).await?;

        let to_pubkey = |b: [u8; 32]| Pubkey::new_from_array(b).to_string();

        let expiration_human =
            chrono::NaiveDateTime::from_timestamp_opt(option.unix_expiration as i64, 0)
                .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "unknown".to_string());

        let strike_price_human = format!(
            "{:.decimals$}",
            option.strike_price as f64 / 10f64.powi(quote_decimals as i32),
            decimals = quote_decimals as usize
        );

        let contract_size_human = format!(
            "{:.decimals$}",
            option.contract_size as f64 / 10f64.powi(underlying_decimals as i32),
            decimals = underlying_decimals as usize
        );

        // ✅ 8. 组装返回
        Ok(OptionParserResponse {
            creator: to_pubkey(option.creator),
            underlying_mint: to_pubkey(option.underlying_mint),
            quote_mint: to_pubkey(option.quote_mint),
            option_mint: to_pubkey(option.option_mint),
            strike_price: option.strike_price.to_string(),
            unix_expiration: option.unix_expiration,
            expiration_human,
            contract_size: option.contract_size.to_string(),
            contract_size_human,
            strike_price_human,
            total_supply,
        })
    }

    /// ✅ 异步获取 token decimals
    async fn get_mint_decimals(&self, mint_pubkey: &Pubkey) -> Result<u8, StatusCode> {
        let mint_account = self
            .rpc
            .get_account(mint_pubkey)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;
        let mint_data = mint_account.data;
        let mint = Mint::unpack(&mint_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(mint.decimals)
    }
}
