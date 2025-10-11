use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::Result;
use borsh::{BorshDeserialize,BorshSerialize,to_vec};
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ExerciseOptionArgs {
    pub exercise_amount: u64,
}
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub fn build_exercise_option_ix(
    program_id: Pubkey,
    exerciser: Pubkey,
    exerciser_option_account: Pubkey,
    exerciser_quote_account: Pubkey,
    exerciser_underlying_account: Pubkey,
    option_account: Pubkey,
    option_mint: Pubkey,
    vault: Pubkey,
    creator_quote_account: Pubkey,
    args: ExerciseOptionArgs,
) -> Result<Instruction> {
    let discriminator: [u8; 8] = [231, 98, 131, 183, 245, 93, 122, 48];

    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);
    ix_data.extend_from_slice(&to_vec(&args)?);

    let accounts = vec![
        AccountMeta::new(exerciser, true), // signer
        AccountMeta::new(exerciser_option_account, false),
        AccountMeta::new(exerciser_quote_account, false),
        AccountMeta::new(exerciser_underlying_account, false),
        AccountMeta::new(option_account, false),
        AccountMeta::new(option_mint, false),
        AccountMeta::new(vault, false),
        AccountMeta::new(creator_quote_account, false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID)?, false),
    ];
   Ok(
     Instruction {
        program_id,
        accounts,
        data: ix_data,
    }
   )
}
