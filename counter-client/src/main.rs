use borsh::{BorshDeserialize, BorshSerialize};
use clap::Parser;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::Transaction,
};

use config::{Config, KeyType, Network};

mod cli;
mod config;

// Define the Counter structure with Borsh serialization and deserialization
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u32,
}

fn main() {
    let cli = cli::Cli::parse();

    let network = match cli.network.as_str() {
        "local" => Network::Local,
        "devnet" => Network::Devnet,
        _ => panic!("Invalid network specified"),
    };

    let key_type = match cli.key_type.as_str() {
        "string" => KeyType::PrivateKey,
        "file" => KeyType::KeypairFile,
        _ => panic!("Invalid key type specified"),
    };

    let config = Config::new(network, key_type, &cli.key_value);
    let client = config.rpc_client();

    // Load the user keypair and program ID from config
    let user = &config.user_keypair;
    let program_id = &config.program_id;

    // Find the Program Derived Address (PDA) for the counter account
    let (counter_pda, _bump_seed) =
        Pubkey::find_program_address(&[user.pubkey().as_ref()], program_id);

    if cli.initialize {
        // Initialize the counter if the init flag is set
        if client.get_account(&counter_pda).is_err() {
            // If the account does not exist, initialize it
            let init_instruction = Instruction {
                program_id: *program_id,
                accounts: vec![
                    AccountMeta::new(user.pubkey(), true), // User's account (signer)
                    AccountMeta::new(counter_pda, false),  // Counter PDA account (to be created)
                    AccountMeta::new_readonly(system_program::ID, false), // System program (for account creation)
                ],
                data: vec![0], // 0 indicates initialization
            };

            let init_tx = Transaction::new_signed_with_payer(
                &[init_instruction],
                Some(&user.pubkey()),
                &[user],
                client.get_latest_blockhash().unwrap(),
            );

            client.send_and_confirm_transaction(&init_tx).unwrap();
            println!("Counter initialized");
        } else {
            println!("Counter account already exists");
        }
    } else if cli.increment {
        // Prepare data for the increment instruction
        let data = vec![1]; // 1 indicates increment
                            // Create an instruction to increment the counter
        let increment_instruction = Instruction {
            program_id: *program_id,
            accounts: vec![
                AccountMeta::new(user.pubkey(), true), // User's account (signer)
                AccountMeta::new(counter_pda, false),  // Counter PDA account
                AccountMeta::new_readonly(system_program::ID, false), // System program
            ],
            data,
        };

        // Create a transaction to send the increment instruction
        let increment_tx = Transaction::new_signed_with_payer(
            &[increment_instruction],
            Some(&user.pubkey()),
            &[user],
            client.get_latest_blockhash().unwrap(),
        );

        // Send and confirm the transaction
        client.send_and_confirm_transaction(&increment_tx).unwrap();
        println!("Counter incremented");
    } else if cli.check {
        // Read the counter value from the counter PDA account
        let account_data = client.get_account_data(&counter_pda).unwrap();
        let counter: Counter = Counter::try_from_slice(&account_data).unwrap();
        println!("Counter value: {}", counter.count);
    }
}
