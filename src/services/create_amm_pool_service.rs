
use crate::clients::create_amm_pool::*;
use crate::dto::create_amm_pool::{CreateAmmPoolRequest, CreateAmmPoolResponse};
use crate::utils::sdk_instructions::to_sdk_instruction;
use crate::utils::to_pubkey::to_pubkey;
use anyhow::Result;
use bincode::config::standard;
use bincode::serde::encode_to_vec;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
pub struct CreateAmmPoolService {
    rpc: RpcClient,
    program_id: Pubkey,
}
impl CreateAmmPoolService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }

    pub async fn build_create_amm_pool_tx(
        &self,
        req: CreateAmmPoolRequest,
    ) -> Result<CreateAmmPoolResponse> {
        let option_mint = req.mint_a;
        let mint_account = self.rpc.get_account(&option_mint).await?;
        let mint_state = Mint::unpack(&mint_account.data)?;
        let option_account_pubkey = mint_state
            .mint_authority
            .ok_or_else(|| anyhow::anyhow!("Mint has no authority"))?;
        let option_account =
            solana_sdk::pubkey::Pubkey::new_from_array(option_account_pubkey.to_bytes());
        //derive pool account
        let (pool_account, _bump) = Pubkey::find_program_address(
            &[
                b"pool",
                &req.mint_a.as_ref(),
                &req.mint_b.as_ref(),
                &req.creator.as_ref(),
            ],
            &self.program_id,
        );
        //fetch pool account token associated account
        let pool_account_pubkey = to_pubkey(pool_account.to_bytes());
        let mint_a_pubkey = to_pubkey(req.mint_a.to_bytes());
        let mint_b_pubkey = to_pubkey(req.mint_b.to_bytes());
        let vault_a_ata = get_associated_token_address(&pool_account_pubkey, &mint_a_pubkey);
        let vault_b_ata = get_associated_token_address(&pool_account_pubkey, &mint_b_pubkey);
        let vault_a = solana_sdk::pubkey::Pubkey::new_from_array(vault_a_ata.to_bytes());
        let vault_b = solana_sdk::pubkey::Pubkey::new_from_array(vault_b_ata.to_bytes());
        //create instruction
        let ix = build_create_amm_pool_ix(
            self.program_id,
            req.creator,
            req.mint_a,
            req.mint_b,
            option_account,
            pool_account,
            vault_a,
            vault_b
        )?;
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&[ix], Some(&req.creator));
        tx.message.recent_blockhash = recent_blockhash;
        let serialized_tx = encode_to_vec(&tx, standard())?;
        let unsigned_tx_base64 = base64::encode(&serialized_tx);
        Ok(CreateAmmPoolResponse {
            unsigned_tx: unsigned_tx_base64,
            pool_account: pool_account.to_string(),
        })
    }
}
