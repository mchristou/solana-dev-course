use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::keypair::Keypair,
    signer::Signer,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
use std::u8;

fn initialize_key_pair() -> Keypair {
    let private_key_env = std::env::var("PRIVATE_KEY").unwrap();

    let private_key: Vec<_> =
        private_key_env.split(',').flat_map(u8::from_str).collect();

    Keypair::from_bytes(private_key.as_slice()).unwrap()
}

#[derive(BorshSerialize, BorshDeserialize)]
struct StudentIntro {
    variant: u8,
    name: String,
    message: String,
}

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);
    let keypair = initialize_key_pair();

    let program_id =
        Pubkey::from_str("HdE95RSVsdb315jfJtaykXhXY478h53X6okDupVfY9yf")
            .unwrap();
    let pda =
        Pubkey::find_program_address(&[keypair.pubkey().as_ref()], &program_id)
            .0;

    let student_intro_instruction = StudentIntro {
        variant: 0,
        name: "Student".to_string(),
        message: "Student message".to_string(),
    };

    let instruction = Instruction::new_with_borsh(
        program_id,
        &student_intro_instruction,
        vec![
            AccountMeta::new_readonly(keypair.pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[&keypair],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(res) => {
            println!("Transaction submitted: https://explorer.solana.com/tx/{res}?cluster=devnet")
        }
        Err(e) => println!("Error: {e}"),
    };

    Ok(())
}
