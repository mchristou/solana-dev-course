use borsh::{BorshDeserialize, BorshSerialize};
use solana_account_decoder::UiDataSliceConfig;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::u8;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct StudentIntro {
    variant: u8,
    name: String,
    message: String,
}

fn prefetch_accounts(client: &RpcClient) -> Vec<Pubkey> {
    let program_id = Pubkey::from_str("HdE95RSVsdb315jfJtaykXhXY478h53X6okDupVfY9yf").unwrap();

    let config = RpcProgramAccountsConfig {
        filters: None,
        account_config: RpcAccountInfoConfig {
            encoding: Some(solana_account_decoder::UiAccountEncoding::Base64),
            data_slice: Some(UiDataSliceConfig {
                offset: 0,
                length: 16,
            }),
            commitment: None,
            min_context_slot: None,
        },
        with_context: None,
    };

    let accounts = client
        .get_program_accounts_with_config(&program_id, config)
        .unwrap();

    accounts.iter().map(|a| a.0).collect()
}

fn fetch_page(client: &RpcClient, page: usize, per_page: usize) -> Vec<StudentIntro> {
    let accounts = prefetch_accounts(client);

    let paginated_keys = &accounts[((page - 1) * per_page)..(page * per_page)];

    let mut students = client
        .get_multiple_accounts(paginated_keys)
        .unwrap()
        .iter()
        .map(|a| StudentIntro::deserialize(&mut a.clone().unwrap().data.as_slice()).unwrap())
        .collect::<Vec<_>>();

    students.sort_by_key(|s| s.name.to_lowercase());
    students
}

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);

    let students = fetch_page(&client, 1, 10);
    println!("{students:?}");

    Ok(())
}
