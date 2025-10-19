use crate::clients::mint_option::{MintOptionArgs, build_mint_option_ix};
use crate::dto::mint_option::{MintOptionRequest, MintOptionResponse, OptionAccount};
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
pub struct MintOptionService {
    rpc: RpcClient,
    program_id: Pubkey,
}
impl MintOptionService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }
    pub async fn build_mint_option_tx(&self, req: MintOptionRequest) -> Result<MintOptionResponse> {
        let args = MintOptionArgs {
            mint_amount: req.mint_amount,
        };
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
        let option_account = OptionAccount::try_from_slice(&option_account_data[8..]).unwrap();
        let unix_expiration = option_account.unix_expiration;
        let unix_now = chrono::Utc::now().timestamp();
        if unix_now > unix_expiration {
            bail!(
                "Option expired: now={} > expiration={}",
                unix_now,
                unix_expiration
            );
        }
        let minter_pubkey = to_pubkey(req.minter.to_bytes());
        let underlying_pubkey = to_pubkey(option_account.underlying_mint.to_bytes());
        let quote_pubkey = to_pubkey(option_account.quote_mint.to_bytes());
        let option_pubkey = to_pubkey(option_account.option_mint.to_bytes());
        let minter_underlying_account_pubkey =
            get_associated_token_address(&minter_pubkey, &underlying_pubkey);
        let minter_quote_account_pubkey =
            get_associated_token_address(&minter_pubkey, &quote_pubkey);
        let minter_option_account_pubkey =
            get_associated_token_address(&minter_pubkey, &option_pubkey);
        let minter_underlying_account =
            solana_sdk::pubkey::Pubkey::new_from_array(minter_underlying_account_pubkey.to_bytes());
        let minter_quote_account =
            solana_sdk::pubkey::Pubkey::new_from_array(minter_quote_account_pubkey.to_bytes());
        let minter_option_account =
            solana_sdk::pubkey::Pubkey::new_from_array(minter_option_account_pubkey.to_bytes());
        let vault_ata = get_associated_token_address(&option_account_pubkey, &underlying_pubkey);
        let vault = solana_sdk::pubkey::Pubkey::new_from_array(vault_ata.to_bytes());
        let ix_main = build_mint_option_ix(
            self.program_id,
            req.minter,
            req.option_mint,
            option_account_address,
            option_account.underlying_mint,
            minter_underlying_account,
            minter_option_account,
            vault,
            args,
        )?;
        let mut ixs = vec![];
        ixs.push(ix_main);
        let recent_blockhash: Hash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&ixs, Some(&req.minter));
        tx.message.recent_blockhash = recent_blockhash;
        //encode tx to base64
        let bytes = encode_to_vec(&tx, standard())?;
        let base64_tx = base64::encode(bytes);
        Ok(MintOptionResponse {
            unsigned_tx: base64_tx,
            mint_amount: req.mint_amount,
        })
    }
}
