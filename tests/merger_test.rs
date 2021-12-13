#![cfg(feature = "test-bpf")]

use std::assert_eq;

use SolNFT::instruction::InitMergerArgs;

mod helpers;

use {
    helpers::*,
    solana_program_test::*,
    market_program::{
        instruction::{PostLiquidityArgs, InitializeMarketArgs},
        processor::Processor,
        state::Market,
        ledger::{Ledger, Position, NUM_ROWS},
        id,
    },
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        program_pack::Pack,

    },
    
    solana_sdk::{
        signature::{Signer, Keypair},
        transaction::Transaction,
        account::Account,
    },

    // spl_token::{
    //     instruction::{initialize_mint, initialize_account},
    // },
    arrayref::array_ref,
    spl_token::instruction::{transfer, mint_to_checked},
    spl_token::{
        state::Account as TokenAccount,
    },
};

#[tokio::test]
async fn test_init_merger() {
    let mut p_test = ProgramTest::new(
        "SolNFT",
        id(),
        processor!(Processor::process),
    );

    let user = Keypair::new();

    p_test.add_account(
        user.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Acount::default()
        }
    );

    let (mut banks_client, payer, recent_blockhash) = p_test.start().await;

    let (
        initializer_pubkey,
        merger_state_account_pubkey,
        nft_mint_a_pubkey,
        nft_mint_b_pubkey,
    ) = create_and_initialize_merger(
        &mut banks_client,
        &payer,
        &recent_blockhash
    ).await;

    let (create_user_nft_account_ix, user_nft_token_account_pubkey) = create_token_account_ix(&user);
    let initialize_user_nft_token_account_ix = initialize_token_account_ix(&user.pubkey(), &nft_mint_a_pubkey, nft)
    //TODO: Finish lol
}