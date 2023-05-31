use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::{sol_to_lamports, LAMPORTS_PER_SOL};
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::{self};
use solana_sdk::transaction::Transaction;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey, signer::keypair::Keypair};
use std::str::FromStr;
use std::u8;

fn initialize_key_pair() -> Keypair {
    let private_key_env = std::env::var("PRIVATE_KEY").unwrap();

    let private_key: Vec<_> = private_key_env.split(',').flat_map(u8::from_str).collect();

    Keypair::from_bytes(private_key.as_slice()).unwrap()
}

fn send_sol(client: RpcClient, amount: u64, to: Pubkey, from: Keypair) {
    let instruction = system_instruction::transfer(&from.pubkey(), &to, amount);

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&from.pubkey()),
        &[&from],
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
    let pubkey = Pubkey::from_str("CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN").unwrap();

    /* Read data from the Solana network */
    let lam = client.get_balance(&pubkey).unwrap();
    println!("Balance: {}", lamports_to_sol(lam));

    let account = client.get_account(&pubkey).unwrap();
    println!("Is account executable? {}", account.executable);

    /* Write data to the Solana network */
    let payer = initialize_key_pair();

    client
        .request_airdrop(&payer.pubkey(), LAMPORTS_PER_SOL)
        .unwrap();
    let lamports = sol_to_lamports(0.1);
    send_sol(client, lamports, Keypair::new().pubkey(), payer);

    Ok(())
}
