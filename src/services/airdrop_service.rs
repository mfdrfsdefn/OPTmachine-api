use crate::dto::airdrop::{AirdropRequest, AirdropResponse};
use crate::utils::sdk_instructions::to_sdk_instruction;
use crate::utils::to_pubkey::to_pubkey;
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_token::instruction::transfer_checked;
use spl_token::solana_program::program_pack::Pack;
use std::str::FromStr;
use std::sync::Arc;
pub struct AirdropService {
    rpc: RpcClient,
    airdrop_wallet: Arc<Keypair>,
}

impl AirdropService {
    pub fn new(rpc_url: &str, airdrop_wallet: Arc<Keypair>) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self {
            rpc,
            airdrop_wallet,
        }
    }

    pub async fn build_airdrop_tx(&self, req: AirdropRequest) -> Result<AirdropResponse> {
        let mint_a = Pubkey::from_str("CRLQpVF8E6bqoyFWX9tv9eiwfUekYYYbGFBeawGuTrR2")?;
        let mint_b = Pubkey::from_str("7URnBJmozJjCi31oMH6ju6joM5LwaLa49NK2aVJfBZYz")?;

        let user = req.user;
        let payer = &*self.airdrop_wallet;
        let user_pubkey = to_pubkey(user.to_bytes());
        let payer_address = payer.pubkey();
        let payer_pubkey = to_pubkey(payer_address.to_bytes());
        let mut ixs: Vec<Instruction> = vec![];
        let mint_a_pubkey = to_pubkey(mint_a.to_bytes());
        let mint_b_pubkey = to_pubkey(mint_b.to_bytes());

        for mint in [mint_a_pubkey, mint_b_pubkey] {
            let user_ata = get_associated_token_address(&user_pubkey, &mint);
            let payer_ata = get_associated_token_address(&payer_pubkey, &mint);
            let user_ata_address = solana_sdk::pubkey::Pubkey::new_from_array(user_ata.to_bytes());

            if self.rpc.get_account(&user_ata_address).await.is_err() {
                println!("🆕 Creating ATA for user: {user_ata} ({mint})");
                let ata_ix = create_associated_token_account(
                    &payer_pubkey,
                    &user_pubkey,
                    &mint,
                    &TOKEN_PROGRAM_ID,
                );
                ixs.push(to_sdk_instruction(ata_ix));
            }

            let amount = 1_000 * 1_000_000_000;
            let transfer_ix = transfer_checked(
                &TOKEN_PROGRAM_ID,
                &payer_ata,
                &mint,
                &user_ata,
                &payer_pubkey,
                &[],
                amount,
                9,
            )?;
            ixs.push(to_sdk_instruction(transfer_ix));
        }

        let recent_blockhash: Hash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&ixs, Some(&payer_address));
        tx.sign(&[payer], recent_blockhash);

        let sig = self.rpc.send_and_confirm_transaction(&tx).await?;
        println!("✅ Airdrop TX sent! Signature: {}", sig);
        Ok(AirdropResponse { ok: true })
    }
}
