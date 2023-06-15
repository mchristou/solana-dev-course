use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::lamports_to_sol, pubkey::Pubkey};
use std::str::FromStr;

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);
    let pubkey =
        Pubkey::from_str("CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN")
            .unwrap();

    /* Read data from the Solana network */
    let lam = client.get_balance(&pubkey).unwrap();
    println!("Balance: {}", lamports_to_sol(lam));

    let account = client.get_account(&pubkey).unwrap();
    println!("Is account executable? {}", account.executable);

    Ok(())
}
