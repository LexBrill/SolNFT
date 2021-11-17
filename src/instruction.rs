use crate::{
    error::NFTError::{
        // self,
        InvalidInstruction,
    }
};
use std::convert::TryInto;
use solana_program::{program_error::ProgramError, program_pack::{IsInitialized, Pack, Sealed}, pubkey::Pubkey};

// use crate::error::NFTError::InvalidInstruction;

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Clone)]
pub struct InitMergerArgs {
    pub nft_mint_a: Pubkey,
    pub nft_mint_b: Pubkey,
}
pub enum NFTInstruction {
    /// 0. `[signer]` User initializing the merger
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitMerger {
        data: InitMergerArgs
    },

    RunMerger {

    }
}

impl NFTInstruction{
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitMerger {
                data: Self::unpack_initmerger_data(rest)?,
            },
            1 => Self::RunMerger {

            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_initmerger_data(input: &[u8]) -> Result<InitMergerArgs, ProgramError> {
        let initmerger_args = InitMergerArgs::unpack(input).unwrap();
        Ok(initmerger_args)
    }

}

impl IsInitialized for InitMergerArgs{
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for InitMergerArgs {}

impl Pack for InitMergerArgs {
    const LEN: usize = 64;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, InitMergerArgs::LEN];
        let (
            nft_mint_a_dst,
            nft_mint_b_dst,
        ) = mut_array_refs![dst, 32, 32];

        let InitMergerArgs {
            nft_mint_a,
            nft_mint_b,
        } = self;

        nft_mint_a_dst.copy_from_slice(nft_mint_a.as_ref());
        nft_mint_b_dst.copy_from_slice(nft_mint_b.as_ref());
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, InitMergerArgs::LEN];
        let (
            nft_mint_a,
            nft_mint_b
        ) = array_refs![src, 32, 32];

        Ok(InitMergerArgs {
            nft_mint_a: Pubkey::new_from_array(*nft_mint_a),
            nft_mint_b: Pubkey::new_from_array(*nft_mint_b),
        })

    }
}