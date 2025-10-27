use crate::clients::reclaim_asset::build_reclaim_asset_ix;
use crate::dto::reclaim_asset::{OptionAccount, ReclaimAssetRequest, ReclaimAssetResponse};
use crate::utils::to_pubkey::to_pubkey;
use anyhow::{Result, bail};
use bincode::config::standard;
use bincode::serde::encode_to_vec;
use borsh::{BorshDeserialize, BorshSerialize};
use chrono::Local;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
pub struct ReclaimAssetService {
    rpc: RpcClient,
    program_id: Pubkey,
}
impl ReclaimAssetService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }
    pub async fn build_reclaim_asset_tx(
        &self,
        req: ReclaimAssetRequest,
    ) -> Result<ReclaimAssetResponse> {
        let reclaimer_pubkey = to_pubkey(req.reclaimer.to_bytes());
        let option_mint = req.option_mint;
        let mint_account = self.rpc.get_account(&option_mint).await?;
        let mint_state = Mint::unpack(&mint_account.data)?;
        let option_account_pubkey = mint_state
            .mint_authority
            .ok_or_else(|| anyhow::anyhow!("Mint has no authority"))?;
        let option_account_address =
            solana_sdk::pubkey::Pubkey::new_from_array(option_account_pubkey.to_bytes());
        let option_account_raw = self.rpc.get_account(&option_account_address).await?;
        let option_account_data = option_account_raw.data;
        if option_account_data.len() < 8 {
            tracing::error!(
                "❌ Option account data too short: len = {}, account likely uninitialized or invalid PDA",
                option_account_data.len()
            );
            return Err(anyhow::anyhow!("Invalid or uninitialized option account"));
        }
        let option_account = OptionAccount::try_from_slice(&option_account_data[8..])?;
        let underlying_mint = option_account.underlying_mint;
        let underlying_mint_pubkey = to_pubkey(underlying_mint.to_bytes());
        let recycler_underlying_account_ata =
            get_associated_token_address(&reclaimer_pubkey, &underlying_mint_pubkey);
        let vault_ata =
            get_associated_token_address(&option_account_pubkey, &underlying_mint_pubkey);
        let vault = solana_sdk::pubkey::Pubkey::new_from_array(vault_ata.to_bytes());
        let recycler_underlying_account =
            solana_sdk::pubkey::Pubkey::new_from_array(recycler_underlying_account_ata.to_bytes());
        let ix = build_reclaim_asset_ix(
            self.program_id,
            req.reclaimer,
            option_account_address,
            recycler_underlying_account,
            vault,
        )?;
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;
        let tx = Transaction::new_with_payer(&[ix], Some(&req.reclaimer));
        let serialized_tx = encode_to_vec(&tx, standard())?;
        let unsigned_tx_base64 = base64::encode(&serialized_tx);
        Ok(ReclaimAssetResponse {
            unsigned_tx: unsigned_tx_base64,
        })
    }
}
