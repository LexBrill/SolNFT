use crate::instruction::NFTInstruction;

use spl_token::{
    instruction::{AuthorityType},  
    state::Account,
    state::Mint,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    // log::sol_log_compute_units,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError::{self, AccountAlreadyInitialized, InvalidArgument},
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    instruction::AccountMeta,
    // sysvar::{self, clock::Clock}
};

use std::result::Result;

use std::convert::TryFrom;

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = NFTInstruction::unpack(instruction_data)?;

        match instruction {
            NFTInstruction::InitMerger { data } => {
                msg!("Instruction: InitMerger");
                process_initmerger(
                    program_id,
                    accounts,
                    &data.nft_mint_a,
                    &data.nft_mint_b,
                )
            }
            NFTInstruction::RunMerger{ } => {
                msg!("Instruction: RunMerger");
                process_runmerger(
                    program_id,
                    accounts,
                )
            }
        }
    }
}

fn process_initmerger(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    nft_mint_a: &Pubkey,
    nft_mint_b: &Pubkey,
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let merger_account = next_account_info(account_info_iter)?;
    


    Ok(())
}

fn process_runmerger(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    Ok(())
}