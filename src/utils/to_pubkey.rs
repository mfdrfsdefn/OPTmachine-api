use solana_pubkey::Pubkey;

/// generate old fashioned pubkey
pub fn to_pubkey<T: Into<[u8; 32]>>(input: T) -> Pubkey {
    Pubkey::new_from_array(input.into())
}
