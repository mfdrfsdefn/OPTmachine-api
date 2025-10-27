use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use serde::de::value;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const RENT_SYSVAR_ID: &str = "SysvarRent111111111111111111111111111111111";
#[derive(BorshSerialize, BorshDeserialize)]
pub struct SwapArgs {
    amount_in: u64,
    a_to_b: bool,
}
pub fn build_swap_ix(
    program_id: Pubkey,
    provider: Pubkey,
    pool_account: Pubkey,
    vault_a: Pubkey,
    vault_b: Pubkey,
    provider_token_a_: Pubkey,
    provider_token_b: Pubkey,
    amount_in: u64,
    a_to_b: bool,
) -> Result<Instruction> {
    let discriminator: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
    let args = SwapArgs { amount_in, a_to_b };
    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);
    ix_data.extend_from_slice(&to_vec(&args)?);
    let accounts = vec![
        AccountMeta::new(provider, true),
        AccountMeta::new(pool_account, false),
        AccountMeta::new(vault_a, false),
        AccountMeta::new(vault_b, false),
        AccountMeta::new(provider_token_a_, false),
        AccountMeta::new(provider_token_b, false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(), false),
    ];
    Ok(Instruction {
        program_id,
        accounts,
        data: ix_data,
    })
}
