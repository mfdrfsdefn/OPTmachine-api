use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const RENT_SYSVAR_ID: &str = "SysvarRent111111111111111111111111111111111";
pub fn build_reclaim_asset_ix(
    program_id: Pubkey,
    reclaimer: Pubkey,
    option_account: Pubkey,
    recycler_underlying_account: Pubkey,
    vault: Pubkey,
) -> Result<Instruction> {
    let discriminator: [u8; 8] = [168, 165, 19, 245, 76, 231, 84, 239];
    let mut ix_data = vec![];
    ix_data.extend_from_slice(&discriminator);

    let accounts = vec![
        AccountMeta::new(reclaimer, true), // signer
        AccountMeta::new(option_account, false),
        AccountMeta::new(recycler_underlying_account, false),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(Pubkey::from_str(TOKEN_PROGRAM_ID)?, false),
    ];

    Ok(Instruction {
        program_id,
        accounts,
        data: ix_data,
    })
}
