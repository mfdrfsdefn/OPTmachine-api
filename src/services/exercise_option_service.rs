use crate::clients::exercise_option::{ExerciseOptionArgs, build_exercise_option_ix};
use crate::dto::exercise_option::{ExerciseOptionRequest, ExerciseOptionResponse};
use crate::dto::mint_option::OptionAccount;
use crate::utils::to_pubkey::to_pubkey;
use anyhow::Result;
use bincode::config::standard;
use bincode::serde::encode_to_vec;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::solana_program::program_pack::Pack;
use spl_token::state::Mint;
pub struct ExerciseOptionService {
    rpc: RpcClient,
    program_id: Pubkey,
}
impl ExerciseOptionService {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        let rpc = RpcClient::new(rpc_url.to_string());
        Self { rpc, program_id }
    }
    pub async fn build_exercise_option_tx(
        &self,
        req: ExerciseOptionRequest,
    ) -> Result<ExerciseOptionResponse> {
        let args = ExerciseOptionArgs {
            exercise_amount: req.exercise_amount,
        };
        let exerciser = req.exerciser;
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
        let creator = option_account.creator;
        let quote_mint = option_account.quote_mint;
        let underlying_mint = option_account.underlying_mint;
        let creator_pubkey = to_pubkey(creator.to_bytes());
        let exerciser_pubkey = to_pubkey(exerciser.to_bytes());
        let quote_mint_pubkey = to_pubkey(quote_mint.to_bytes());
        let underlying_mint_pubkey = to_pubkey(underlying_mint.to_bytes());
        let option_mint_pubkey = to_pubkey(option_mint.to_bytes());
        let exerciser_option_account_ata =
            get_associated_token_address(&exerciser_pubkey, &option_mint_pubkey);
        let exerciser_quote_account_ata =
            get_associated_token_address(&exerciser_pubkey, &quote_mint_pubkey);
        let exerciser_underlying_account_ata =
            get_associated_token_address(&exerciser_pubkey, &underlying_mint_pubkey);
        let vault_ata =
            get_associated_token_address(&option_account_pubkey, &underlying_mint_pubkey);
        let creator_quote_account_ata =
            get_associated_token_address(&creator_pubkey, &quote_mint_pubkey);
        let exerciser_option_account = Pubkey::new_from_array(
            get_associated_token_address(&exerciser_pubkey, &option_mint_pubkey).to_bytes(),
        );
        let exerciser_quote_account = Pubkey::new_from_array(
            get_associated_token_address(&exerciser_pubkey, &quote_mint_pubkey).to_bytes(),
        );
        let exerciser_underlying_account = Pubkey::new_from_array(
            get_associated_token_address(&exerciser_pubkey, &underlying_mint_pubkey).to_bytes(),
        );
        let vault = Pubkey::new_from_array(
            get_associated_token_address(&option_account_pubkey, &underlying_mint_pubkey)
                .to_bytes(),
        );
        let creator_quote_account = Pubkey::new_from_array(
            get_associated_token_address(&creator_pubkey, &quote_mint_pubkey).to_bytes(),
        );
        let ix_main = build_exercise_option_ix(
            self.program_id,
            exerciser,
            exerciser_option_account,
            exerciser_quote_account,
            exerciser_underlying_account,
            option_account_address,
            option_mint,
            vault,
            creator_quote_account,
            args,
        )?;
        let mut ixs = vec![];
        ixs.push(ix_main);
        let recent_blockhash: Hash = self.rpc.get_latest_blockhash().await?;
        let mut tx = Transaction::new_with_payer(&ixs, Some(&req.exerciser));
        tx.message.recent_blockhash = recent_blockhash;
        //encode tx to base64
        let bytes = encode_to_vec(&tx, standard())?;
        let base64_tx = base64::encode(bytes);
        Ok(ExerciseOptionResponse {
            unsigned_tx: base64_tx,
            exercise_amount: req.exercise_amount,
        })
    }
}
