use solana_sdk::{
    instruction::{Instruction as SdkInstruction, AccountMeta as SdkAccountMeta},
    pubkey::Pubkey as SdkPubkey,
};

pub fn to_sdk_instruction<T>(ix: T) -> SdkInstruction
where
    T: Into<ProgramLikeInstruction>,
{
    let ix: ProgramLikeInstruction = ix.into();
    SdkInstruction {
        program_id: SdkPubkey::new_from_array(ix.program_id),
        accounts: ix
            .accounts
            .into_iter()
            .map(|meta| SdkAccountMeta {
                pubkey: SdkPubkey::new_from_array(meta.pubkey),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect(),
        data: ix.data,
    }
}

pub struct ProgramLikeInstruction {
    pub program_id: [u8; 32],
    pub accounts: Vec<ProgramLikeAccountMeta>,
    pub data: Vec<u8>,
}

pub struct ProgramLikeAccountMeta {
    pub pubkey: [u8; 32],
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<solana_program::instruction::Instruction> for ProgramLikeInstruction {
    fn from(ix: solana_program::instruction::Instruction) -> Self {
        Self {
            program_id: ix.program_id.to_bytes(),
            accounts: ix
                .accounts
                .into_iter()
                .map(|m| ProgramLikeAccountMeta {
                    pubkey: m.pubkey.to_bytes(),
                    is_signer: m.is_signer,
                    is_writable: m.is_writable,
                })
                .collect(),
            data: ix.data,
        }
    }
}

impl From<spl_associated_token_account::solana_program::instruction::Instruction>
    for ProgramLikeInstruction
{
    fn from(ix: spl_associated_token_account::solana_program::instruction::Instruction) -> Self {
        Self {
            program_id: ix.program_id.to_bytes(),
            accounts: ix
                .accounts
                .into_iter()
                .map(|m| ProgramLikeAccountMeta {
                    pubkey: m.pubkey.to_bytes(),
                    is_signer: m.is_signer,
                    is_writable: m.is_writable,
                })
                .collect(),
            data: ix.data,
        }
    }
}
