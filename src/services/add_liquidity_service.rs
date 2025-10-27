use crate::clients::add_liquidity::*;
use crate::dto::add_liquidity::{AddLiquidityRequest, AddLiquidityResponse, PoolAccount};
use crate::utils::to_pubkey::to_pubkey;
use anyhow::Result;
use bincode::config::standard;
use bincode::serde::encode_to_vec;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::message;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
pub struct AddLiquidityService {
    rpc: RpcClient,
    program_id: Pubkey,
}
impl AddLiquidityService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }
    pub async fn build_add_liquidity_tx(
        &self,
        req: AddLiquidityRequest,
    ) -> Result<AddLiquidityResponse> {
        let provider = req.provider;
        let pool = req.pool;
        let pool_account_raw = self.rpc.get_account(&pool).await?;
        let pool_account_data = pool_account_raw.data;
        let pool_account = PoolAccount::try_from_slice(&pool_account_data[8..]).unwrap();
        let provider_pubkey = to_pubkey(provider.to_bytes());
        let pool_account_pubkey = to_pubkey(pool.to_bytes());
        let mint_a_pubkey = to_pubkey(pool_account.mint_a.to_bytes());
        let mint_b_pubkey = to_pubkey(pool_account.mint_b.to_bytes());
        let vault_a_ata = get_associated_token_address(&pool_account_pubkey, &mint_a_pubkey);
        let vault_b_ata = get_associated_token_address(&pool_account_pubkey, &mint_b_pubkey);
        let provider_a_ata = get_associated_token_address(&provider_pubkey, &mint_a_pubkey);
        let provider_b_ata = get_associated_token_address(&provider_pubkey, &mint_b_pubkey);
        let vault_a = solana_sdk::pubkey::Pubkey::new_from_array(vault_a_ata.to_bytes());
        let vault_b = solana_sdk::pubkey::Pubkey::new_from_array(vault_b_ata.to_bytes());
        let provider_token_a =
            solana_sdk::pubkey::Pubkey::new_from_array(provider_a_ata.to_bytes());
        let provider_token_b =
            solana_sdk::pubkey::Pubkey::new_from_array(provider_b_ata.to_bytes());
        let ix = build_add_liquidity_ix(
            self.program_id,
            provider,
            pool,
            vault_a,
            vault_b,
            provider_token_a,
            provider_token_b,
            req.deposit_a,
        )?;
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&[ix], Some(&provider));
        tx.message.recent_blockhash = recent_blockhash;
        let bytes = encode_to_vec(&tx, standard())?;
        let base64_tx = base64::encode(bytes);
        Ok(AddLiquidityResponse {
            unsigned_tx: base64_tx,
        })
    }
}
