use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const RENT_SYSVAR_ID: &str = "SysvarRent111111111111111111111111111111111";

pub fn build_create_amm_pool_ix(
    program_id: Pubkey,
    creator: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    option_account: Pubkey,
    pool: Pubkey,
    vault_a: Pubkey,
    vault_b: Pubkey,
) -> Result<Instruction> {
    let discriminator: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];

    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);
    let accounts = vec![
        AccountMeta::new(creator, true),
        AccountMeta::new_readonly(mint_a, false),
        AccountMeta::new_readonly(mint_b, false),
        AccountMeta::new_readonly(option_account, false),
        AccountMeta::new(pool, false),
        AccountMeta::new(vault_a, false),
        AccountMeta::new(vault_b, false),
        AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(), false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap(), false),
        AccountMeta::new_readonly(
            Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID).unwrap(),
            false,
        ),
        AccountMeta::new_readonly(Pubkey::from_str(RENT_SYSVAR_ID).unwrap(), false),
    ];
    Ok(Instruction {
        program_id,
        accounts,
        data: ix_data,
    })
}
