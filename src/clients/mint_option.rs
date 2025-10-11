use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::Result;
use borsh::{BorshDeserialize,BorshSerialize,to_vec};
#[derive(BorshSerialize, BorshDeserialize)]
pub struct MintOptionArgs {
    pub mint_amount: u64,
}
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub fn build_mint_option_ix (
    program_id: Pubkey,
    minter: Pubkey,
    option_mint: Pubkey,
    option_account: Pubkey,
    underlying_mint: Pubkey,
    minter_underlying_account: Pubkey,
    minter_option_account: Pubkey,
    vault: Pubkey,    
    args: MintOptionArgs
) ->Result<Instruction>{
    let discriminator: [u8; 8] = [83,220,200,39,231,140,153,141];//mint option
    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);
    ix_data.extend_from_slice(&to_vec(&args)?);
    let accounts = vec![
        AccountMeta::new(minter, true), // Signer
        AccountMeta::new(option_mint, false),
        AccountMeta::new_readonly(option_account, false),
        AccountMeta::new_readonly(underlying_mint, false),
        AccountMeta::new(minter_underlying_account, false),
        AccountMeta::new(minter_option_account, false),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(), false),
    ];
        Ok(Instruction {
        program_id,
        accounts,
        data: ix_data,
    })  

}