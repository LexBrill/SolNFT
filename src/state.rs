use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey
};


use borsh::{BorshSerialize, BorshDeserialize};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Merger {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    // pub nft_burn_account_a: Pubkey,
    // pub nft_burn_account_b: Pubkey,
    pub nft_mint_a: Pubkey,
    pub nft_mint_b: Pubkey,
}

// impl Sealed for Merger {}

// impl IsInitialized for Merger {
//     fn is_initalized(&self) -> bool {
//         self.is_initialized
//     }
// }

// impl Pack for Merger {
//     const LEN: usize = ;

//     fn pack_into_slice(&self, dst: &mut [u8]) {
//         let dst = array_mut_ref![dst, 0, Merger::LEN];
//         let (
//             is_initialized_dst,
//             initializer_pubkey_dst,
//             nft_mint_a_dst,
//             nft_mint_b_dst,
//         )
//     }
// }