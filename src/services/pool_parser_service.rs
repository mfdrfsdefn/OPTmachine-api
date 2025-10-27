use crate::dto::pool_parser::{PoolAccountResponse, PoolParserRequest};
use axum::http::StatusCode;
use borsh::{BorshDeserialize, BorshSerialize};
use num_format::{Locale, ToFormattedString};
use serde::Serialize;
use sha2::{Digest, Sha256};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::sync::Arc;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PoolAccount {
    pub creator: [u8; 32],
    pub mint_a: [u8; 32],
    pub mint_b: [u8; 32],
    pub amount_a: u64,
    pub amount_b: u64,
    pub k: u128,
    pub unix_expiration: i64,
    pub bump: u8,
}

#[derive(Clone)]
pub struct PoolParserService {
    pub rpc: Arc<RpcClient>,
}

impl PoolParserService {
    pub fn new(rpc_url: &str) -> Self {
        let rpc = Arc::new(RpcClient::new(rpc_url.to_string()));
        Self { rpc }
    }

    pub async fn parse_pool_account(
        &self,
        req: PoolParserRequest,
    ) -> Result<PoolAccountResponse, StatusCode> {
        let pool_pubkey = &req.pool_account;
        let account = self
            .rpc
            .get_account(pool_pubkey)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let data = &account.data;
        let mut hasher = Sha256::new();
        hasher.update("account:PoolAccount");
        let discriminator = &hasher.finalize()[..8];

        if data.len() < 8 || &data[..8] != discriminator {
            return Err(StatusCode::BAD_REQUEST);
        }

        let pool = PoolAccount::try_from_slice(&data[8..])
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let to_pubkey = |b: [u8; 32]| Pubkey::new_from_array(b).to_string();
        let expiration_human = chrono::NaiveDateTime::from_timestamp_opt(pool.unix_expiration, 0)
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "unkown".to_string());

        Ok(PoolAccountResponse {
            creator: to_pubkey(pool.creator),
            mint_a: to_pubkey(pool.mint_a),
            mint_b: to_pubkey(pool.mint_b),
            amount_a: format!("{} ", pool.amount_a.to_formatted_string(&Locale::en)),
            amount_b: format!("{} ", pool.amount_b.to_formatted_string(&Locale::en)),
            k_human: format!("{:.3e}", pool.k as f64),
            unix_expiration: pool.unix_expiration,
            expiration_human,
        })
    }
}
