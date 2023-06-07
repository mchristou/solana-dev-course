use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::u8;

#[derive(BorshSerialize, BorshDeserialize)]
struct StudentIntro {
    variant: u8,
    name: String,
    message: String,
}

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);

    let program_id =
        Pubkey::from_str("HdE95RSVsdb315jfJtaykXhXY478h53X6okDupVfY9yf")
            .unwrap();

    let _: Vec<_> = client
        .get_program_accounts(&program_id)
        .unwrap()
        .iter()
        .inspect(|(_pubkey, account)| {
            let data = StudentIntro::deserialize(&mut account.data.as_slice())
                .unwrap();
            println!("Name: {} - Message: {}", data.name, data.message);
        })
        .collect();

    Ok(())
}
