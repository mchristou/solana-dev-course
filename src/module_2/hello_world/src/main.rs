use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    signer::keypair::Keypair, signer::Signer, transaction::Transaction,
};
use std::str::FromStr;
use std::u8;

fn initialize_key_pair() -> Keypair {
    let private_key_env = std::env::var("PRIVATE_KEY").unwrap();

    let private_key: Vec<_> =
        private_key_env.split(',').flat_map(u8::from_str).collect();

    Keypair::from_bytes(private_key.as_slice()).unwrap()
}

fn say_hello(client: RpcClient, payer: Keypair) {
    let program_id =
        Pubkey::from_str("9whiGXV1XPWdk1BaNTeheey9xuJk37U4amvqDnX1gpcW")
            .unwrap();

    let instruction =
        instruction::Instruction::new_with_bytes(program_id, &[], vec![]);

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&transaction) {
        Ok(sig) => {
            println!("You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/{sig}?cluster=devnet");
        }
        Err(e) => println!("Error: {e}"),
    }
}

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);

    let payer = initialize_key_pair();

    client.request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL).unwrap();

    say_hello(client, payer);

    Ok(())
}
