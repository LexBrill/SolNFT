use solana_program::pubkey::Pubkey;

pub struct Merger {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub nft_mint_a: Pubkey,
    pub nft_mint_b: Pubkey,
}