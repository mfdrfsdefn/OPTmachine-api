use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
#[derive(BorshSerialize, BorshDeserialize)]
pub struct CreateOptionCallArgs {
    pub strike_price: u64,
    pub unix_expiration: i64,
    pub contract_size: u64,
}

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const RENT_SYSVAR_ID: &str = "SysvarRent111111111111111111111111111111111";

pub fn build_create_option_ix(
    program_id: Pubkey,
    creator: Pubkey,
    vault: Pubkey,
    option_mint: Pubkey,
    underlying_mint: Pubkey,
    quote_mint: Pubkey,
    option_account: Pubkey,
    creator_option_account: Pubkey,
    args: CreateOptionCallArgs,
) -> Result<Instruction> {
    let discriminator: [u8; 8] = [38, 48, 104, 254, 14, 229, 81, 122]; // create_option_call

    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);
    ix_data.extend_from_slice(&to_vec(&args)?);

    let accounts = vec![
        AccountMeta::new(creator, true),
        AccountMeta::new(vault, false),
        AccountMeta::new(option_mint, true),
        AccountMeta::new(underlying_mint, false),
        AccountMeta::new(quote_mint, false),
        AccountMeta::new(option_account, false),
        AccountMeta::new(creator_option_account, false),
        AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(RENT_SYSVAR_ID).unwrap(), false),
        AccountMeta::new_readonly(
            Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID).unwrap(),
            false,
        ),
    ];

    Ok(Instruction {
        program_id,
        accounts,
        data: ix_data,
    })
}
