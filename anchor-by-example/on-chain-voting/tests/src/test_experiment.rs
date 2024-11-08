// Interesting discussion on rust test in anchor https://github.com/coral-xyz/anchor/pull/2805
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster, Program,
};

use on_chain_voting::{self, VoteBank};
use std::sync::{Arc, LazyLock, Mutex};

// In case we want to share the same vote bank between tests
// Source: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| {
    let anchor_wallet = std::env::var("ANCHOR_WALLET").unwrap();
    Mutex::new(Config {
        payer: Arc::new(read_keypair_file(&anchor_wallet).unwrap()),
        program_id: on_chain_voting::id(),
        vote_bank: None,
    })
});

#[derive(Debug)]
struct Config {
    payer: Arc<Keypair>,
    program_id: Pubkey,
    vote_bank: Option<Keypair>,
}

fn setup() -> Program<Arc<Keypair>> {
    // Set up program and client
    let mut locked_config = CONFIG.lock().unwrap();
    let client = Client::new_with_options(
        Cluster::Localnet,
        locked_config.payer.clone(),
        CommitmentConfig::confirmed(),
    );
    
    let program = client.program(locked_config.program_id).unwrap();

    // only initialize vote bank once
    if locked_config.vote_bank.is_none() {
        locked_config.vote_bank = Some(Keypair::new());

        let vote_bank = locked_config.vote_bank.as_ref().unwrap();

        // Initialize vote bank
        let tx = program
            .request()
            .accounts(on_chain_voting::accounts::InitVote {
                vote_account: vote_bank.pubkey(),
                signer: locked_config.payer.pubkey(),
                system_program: solana_program::system_program::ID,
            })
            .signer(vote_bank)
            .args(on_chain_voting::instruction::InitVoteBank {})
            .send()
            .unwrap();

        println!("Transaction signature for initializing vote bank: {}", tx);
    } else {
        println!("Vote bank already initialized");
    }

    program
}

#[test]
fn test_vote_for_gm() {
    let program = setup();
    let locked_config = CONFIG.lock().unwrap();

    let vote_bank_account: VoteBank = program
        .account(locked_config.vote_bank.as_ref().unwrap().pubkey())
        .unwrap();

    assert_eq!(vote_bank_account.gm, 0);

    program
        .request()
        .accounts(on_chain_voting::accounts::GibVote {
            vote_account: locked_config.vote_bank.as_ref().unwrap().pubkey(),
            signer: locked_config.payer.pubkey(),
        })
        .signer(locked_config.payer.as_ref())
        .args(on_chain_voting::instruction::GibVote {
            vote_type: on_chain_voting::VoteType::GM,
        })
        .send()
        .unwrap();

    let vote_bank_account: VoteBank = program
        .account(locked_config.vote_bank.as_ref().unwrap().pubkey())
        .unwrap();

    assert_eq!(vote_bank_account.gm, 1);
}

#[test]
fn test_vote_for_gn() {
    let program = setup();
    let locked_config = CONFIG.lock().unwrap();

    program
        .request()
        .accounts(on_chain_voting::accounts::GibVote {
            vote_account: locked_config.vote_bank.as_ref().unwrap().pubkey(),
            signer: locked_config.payer.pubkey(),
        })
        .signer(locked_config.payer.as_ref())
        .args(on_chain_voting::instruction::GibVote {
            vote_type: on_chain_voting::VoteType::GN,
        })
        .send()
        .unwrap();

    let vote_bank_account: VoteBank = program
        .account(locked_config.vote_bank.as_ref().unwrap().pubkey())
        .unwrap();

    assert_eq!(vote_bank_account.gm, 1);
    assert_eq!(vote_bank_account.gn, 1);
}
