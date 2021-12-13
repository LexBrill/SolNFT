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

use borsh::{BorshSerialize, BorshDeserialize};

use crate::state::Merger;

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
                    // &data.nft_mint_a,
                    // &data.nft_mint_b,
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
    // nft_mint_a: &Pubkey,
    // nft_mint_b: &Pubkey,
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let merger_state_account = next_account_info(account_info_iter)?;
    let nft_mint_a = next_account_info(account_info_iter)?;
    let nft_mint_b = next_account_info(account_info_iter)?;

    // let 

    // pass in two accounts, create a PDA and give ownership of the two NFT accounts to the PDA
    // How do I then reference the PDA in the process_runmerger code in order to burn the NFt's and mint one back

    // all i need is a seed, such as pubkey of new mint
    let mut merger_state_account_struct = Merger::try_from_slice(&merger_state_account.data.borrow_mut())?;
    merger_state_account_struct.initializer_pubkey = *initializer.key;
    merger_state_account_struct.is_initialized = true;
    merger_state_account_struct.nft_mint_a = *nft_mint_a.key;
    merger_state_account_struct.nft_mint_b = *nft_mint_b.key;
    merger_state_account_struct.try_to_vec().unwrap();
    


    Ok(())
}

fn process_runmerger(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let caller = next_account_info(account_info_iter)?;
    let merger_state_account = next_account_info(account_info_iter)?;
    let caller_nft_account_a = next_account_info(account_info_iter)?;
    let caller_nft_account_b = next_account_info(account_info_iter)?;
    let nft_burn_account_a = next_account_info(account_info_iter)?;
    let nft_burn_account_b = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let new_nft_mint = next_account_info(account_info_iter)?;
    let new_nft_account = next_account_info(account_info_iter)?;

    let mut merger_state_account_struct = Merger::try_from_slice(&merger_state_account.data.borrow_mut())?;
    let caller_nft_account_a_info = Account::unpack(&caller_nft_account_a.data.borrow())?;
    let caller_nft_account_b_info = Account::unpack(&caller_nft_account_b.data.borrow())?;

    if !merger_state_account_struct.is_initialized{
        //error
    }
    if caller_nft_account_a_info.mint != merger_state_account_struct.nft_mint_a { 
        //error
    }
    if caller_nft_account_b_info.mint != merger_state_account_struct.nft_mint_b {
        //error
    }
    // if nft_burn_account_a.key != &merger_state_account_struct.nft_burn_account_a {
    //     //error
    // }
    // if nft_burn_account_b.key != &merger_state_account_struct.nft_burn_account_b {
    //     //error
    // }
    // check rent?
    // check if owner of the caller nft accounts are the caller
    // check to see if the owner of the 
    
    let burn_nft_a = spl_token::instruction::burn(
        token_program.key,
        caller_nft_account_a.key,
        &merger_state_account_struct.nft_mint_a,
        caller.key,
        &[&caller.key],
        1
    )?;
    invoke(
        &burn_nft_a,
        &[
            caller_nft_account_a.clone(),
            caller.clone(),
            token_program.clone(),
        ]
    )?;

    let burn_nft_b = spl_token::instruction::burn(
        token_program.key,
        caller_nft_account_b.key,
        &merger_state_account_struct.nft_mint_b,
        caller.key,
        &[&caller.key],
        1
    )?;
    invoke(
        &burn_nft_b,
        &[
            caller_nft_account_b.clone(),
            caller.clone(),
            token_program.clone(),
        ]
    )?;

    let mint_new_nft = spl_token::instruction::mint_to(
        token_program.key,
        new_nft_mint.key,
        new_nft_account.key,
        caller.key,
        &[&caller.key],
        1
    )?;
    invoke(
        &mint_new_nft,
        &[
            new_nft_mint.clone(),
            new_nft_account.clone(),
            caller.clone(),
            token_program.clone(),
        ]
    )?;

    // let 

    // // more checks
    // let transfer_to_nft_burn_account_a = spl_token::instruction::transfer(
    //     token_program.key,
    //     caller_nft_account_a.key,
    //     &merger_state_account_struct.nft_burn_account_a,
    //     caller.key,
    //     &[&caller.key],
    //     1
    // )?;
    // invoke(
    //     &transfer_to_nft_burn_account_a,
    //     &[
    //         caller_nft_account_a.clone(),
    //         nft_burn_account_a.clone(),
    //         caller.clone(),
    //         token_program.clone()
    //     ]
    // )?;

    // let transfer_to_nft_burn_account_b = spl_token::instruction::transfer(
    //     token_program.key,
    //     caller_nft_account_b.key,
    //     &merger_state_account_struct.nft_burn_account_b,
    //     caller.key,
    //     &[&caller.key],
    //     1
    // )?;
    // invoke(
    //     &transfer_to_nft_burn_account_b,
    //     &[
    //         caller_nft_account_b.clone(),
    //         nft_burn_account_b.clone(),
    //         caller.clone(),
    //         token_program.clone()
    //     ]
    // )?;

    // make a PDA and make it mint a token

    // use the PDA to transfer the NFT to the user account
        

    Ok(())
}

// Next step
// similar funciton but instead of burning and merging into a new one
    // have them breed in a new way? 
    // maybe only be able to breed them once?