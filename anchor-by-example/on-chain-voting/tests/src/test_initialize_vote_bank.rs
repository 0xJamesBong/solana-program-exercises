// Interesting discussion on rust test in anchor https://github.com/coral-xyz/anchor/pull/2805

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster,
};
use on_chain_voting::{self, accounts, instruction};
use std::str::FromStr; // Adjust the import to your module path

#[test]
fn test_initialize_vote_bank() {
    // Set up program and client
    let program_id = "4c1xHGDjPz6DMzDRvTedYJzUc4yAdKbsYRSFNxbQrSSr";
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Localnet, &payer, CommitmentConfig::confirmed());

    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
    let vote_bank = Keypair::new();

    let tx = program
        .request()
        .accounts(on_chain_voting::accounts::InitVote {
            vote_account: vote_bank.pubkey(),
            signer: payer.pubkey(),
            system_program: solana_program::system_program::ID,
        })
        .signer(&vote_bank)
        .args(on_chain_voting::instruction::InitVoteBank {})
        .send()
        .expect("");

    println!("Transaction signature for initializing vote bank: {}", tx);

    // return program;
}

// fn test_vote_for_gm() {
//     test_initialize_vote_bank();
// }
