use solana_sdk::signer::Signer;
use solana_sdk::{
    transaction::Transaction,
    hash::Hash,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::Keypair,
};
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use crate::dto::create_option::{CreateOptionRequest,CreateOptionResponse};
use crate::clients::create_option::*; // build_create_option_ix, CreateOptionCallArgs
use spl_associated_token_account::get_associated_token_address;
use crate::utils::to_pubkey::to_pubkey;
use bincode::serde::encode_to_vec;
use bincode::config::standard;
pub struct CreateOptionService {
    rpc: RpcClient,
    program_id: Pubkey
}

impl CreateOptionService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }

    pub async fn build_create_option_tx(
        &self,
        req: CreateOptionRequest,
    ) -> Result<CreateOptionResponse> {
        //derive option account
        let (option_account,bump) = Pubkey::find_program_address(
                &[
                    b"optmachine",
                    &req.creator.as_ref(),
                    &req.underlying_mint.as_ref(),
                    &req.quote_mint.as_ref(),
                    &req.strike_price.to_le_bytes(),
                    &req.unix_expiration.to_le_bytes(),
                ],
            
            &self.program_id);
            //fetch option_account token associated account
            let option_account_pubkey = to_pubkey(option_account.to_bytes());
            let underlyingmint_pubkey = to_pubkey(req.underlying_mint.to_bytes());
            let vault_ata = get_associated_token_address(
    &option_account_pubkey,
    &underlyingmint_pubkey,
        );
        let vault = solana_sdk::pubkey::Pubkey::new_from_array(vault_ata.to_bytes());
        //create option mint keypair
        let option_mint_keypair =Keypair::new();
        let option_mint = option_mint_keypair.pubkey();

        // fetch creator option token associated account
        let creator_pubkey = to_pubkey(req.creator.to_bytes());
        let option_mint_pubkey = to_pubkey(option_mint.to_bytes());
        let creator_option_ata = get_associated_token_address(&creator_pubkey, &option_mint_pubkey);
        let creator_option_account = solana_sdk::pubkey::Pubkey::new_from_array(creator_option_ata.to_bytes());
        let args = CreateOptionCallArgs {
            strike_price: req.strike_price,
            unix_expiration: req.unix_expiration,
            contract_size: req.contract_size,
        };
    
        let ix: Instruction = build_create_option_ix(
            self.program_id,
            req.creator,
            vault,
            option_mint,
            req.underlying_mint,
            req.quote_mint,
            option_account,
            creator_option_account,
            args,
        )?;

        //fetch recent blockhash
        let recent_blockhash: Hash = self.rpc.get_latest_blockhash().await?;

        let mut tx = Transaction::new_with_payer(&[ix], Some(&req.creator));
        tx.message.recent_blockhash = recent_blockhash;
        tx.partial_sign(&[&option_mint_keypair], recent_blockhash);
        //encode tx to base64
        let bytes = encode_to_vec(&tx, standard())?;
        let base64_tx = base64::encode(bytes);
        Ok(CreateOptionResponse{
            unsigned_tx: base64_tx,
            option_token_mint: option_mint.to_string(),
        })
    }
}
