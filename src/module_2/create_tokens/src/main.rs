use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token::{
    instruction,
    state::{Account, Mint},
};
use std::str::FromStr;
use std::u8;

fn initialize_key_pair() -> Keypair {
    let private_key_env = std::env::var("PRIVATE_KEY").unwrap();

    let private_key: Vec<_> =
        private_key_env.split(',').flat_map(u8::from_str).collect();

    Keypair::from_bytes(private_key.as_slice()).unwrap()
}

fn create_mint(
    client: &RpcClient,
    payer: &Keypair,
    owner: &Keypair,
    mint_account: &Keypair,
) {
    let token_program = &spl_token::id();
    let mint_rent =
        client.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();

    let create_account_instr =
        solana_program::system_instruction::create_account(
            &payer.pubkey(),
            &mint_account.pubkey(),
            mint_rent,
            Mint::LEN as u64,
            token_program,
        );

    let init_mint_instr = instruction::initialize_mint(
        token_program,
        &mint_account.pubkey(),
        &owner.pubkey(),
        None,
        2,
    )
    .unwrap();

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let mint_transaction = Transaction::new_signed_with_payer(
        &[create_account_instr, init_mint_instr],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&mint_transaction) {
        Ok(sig) => {
            println!("Create Mint - View your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/{sig}?cluster=devnet");
        }
        Err(e) => println!("Error: {e}"),
    }
}

fn create_token_account(
    client: &RpcClient,
    payer: &Keypair,
    token_account: &Keypair,
    mint_account: &Keypair,
) {
    let token_program = &spl_token::id();
    let mint_rent =
        client.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap();
    let token_account_instr =
        solana_program::system_instruction::create_account(
            &payer.pubkey(),
            &token_account.pubkey(),
            mint_rent,
            Account::LEN as u64,
            token_program,
        );

    let account = Keypair::new();
    let init_account_instr = instruction::initialize_account(
        token_program,
        &token_account.pubkey(),
        &mint_account.pubkey(),
        &account.pubkey(),
    )
    .unwrap();

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let mint_transaction = Transaction::new_signed_with_payer(
        &[token_account_instr, init_account_instr],
        Some(&payer.pubkey()),
        &[payer, token_account],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&mint_transaction) {
        Ok(sig) => {
            println!("Create token account - View your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/{sig}?cluster=devnet");
        }
        Err(e) => println!("Error: {e}"),
    }
}

fn mint_tokens(
    client: &RpcClient,
    mint_amount: u64,
    payer: &Keypair,
    mint_account: &Keypair,
    token_account: &Keypair,
    owner: &Keypair,
) {
    let token_program = &spl_token::id();
    let mint_instr = instruction::mint_to(
        token_program,
        &mint_account.pubkey(),
        &token_account.pubkey(),
        &owner.pubkey(),
        &[],
        mint_amount,
    )
    .unwrap();

    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let mint_transaction = Transaction::new_signed_with_payer(
        &[mint_instr],
        Some(&payer.pubkey()),
        &[payer, owner],
        latest_blockhash,
    );

    match client.send_and_confirm_transaction(&mint_transaction) {
        Ok(sig) => {
            println!("Mint tokens - View your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/{sig}?cluster=devnet");
        }
        Err(e) => println!("Error: {e}"),
    }
}

fn main() -> web3::Result<()> {
    let url = "https://api.devnet.solana.com";
    let client = RpcClient::new(url);
    let payer = initialize_key_pair();

    let mint_account = Keypair::new();
    let owner = Keypair::new();
    let token_account = Keypair::new();

    create_mint(&client, &payer, &owner, &mint_account);

    create_token_account(&client, &payer, &token_account, &mint_account);

    let mint_amount = 5;
    mint_tokens(
        &client,
        mint_amount,
        &payer,
        &mint_account,
        &token_account,
        &owner,
    );

    let token_account_info =
        client.get_account(&token_account.pubkey()).unwrap();
    let token_account_data = Account::unpack(&token_account_info.data).unwrap();
    assert_eq!(mint_amount, token_account_data.amount);

    Ok(())
}
