use rust_client::{check_balance, keypair_path_payer};
use solana_sdk::{ account_info, instruction::{AccountMeta, Instruction}, message::Message, program_pack::Pack, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, signers::Signers, system_instruction, transaction::{self, Transaction}};

use solana_client::{client_error::reqwest::Client, rpc_client::RpcClient};
use std::{fs::{self, read}, path::{Path, PathBuf}, str::FromStr};
use borsh::{BorshDeserialize};
use std::error::Error;
use spl_token::{
    id,
    self,
    state::Mint,
    instruction as token_instruction, 
};



fn main() ->  Result<(), Box<dyn Error>>{
    let client = RpcClient::new("http://127.0.0.1:8899".to_string());

    let payer = keypair_path_payer().unwrap();
    let balance = check_balance(&client, payer.pubkey())?;

    let airdrop_signature = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    client.confirm_transaction(&airdrop_signature)?;

    let token_account =  Pubkey::from_str("4PMycudQtomj2fdn6zswVoGc9XAdzmKLSc8Vfr2nffdN")?;
    let rent_exemption = client.get_minimum_balance_for_rent_exemption(Mint::LEN as usize)?;

    let mint = Keypair::new();


    let token_account_pubkey = Pubkey::from_str("4PMycudQtomj2fdn6zswVoGc9XAdzmKLSc8Vfr2nffdN")?;

    let rent_exemption = client.get_minimum_balance_for_rent_exemption(Mint::LEN as usize)?;
    let create_mint_account_tx = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        rent_exemption,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let init_mint_instruction = token_instruction::initialize_mint(
        &spl_token::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        9,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[create_mint_account_tx, init_mint_instruction],
        Some(&payer.pubkey()),
    );
    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer, &mint], recent_blockhash);
    client.send_and_confirm_transaction(&transaction)?;

  
    let token_account = Keypair::new();
    let create_token_account_tx = system_instruction::create_account(
        &payer.pubkey(),
        &token_account.pubkey(),
        client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN as usize)?,
        spl_token::state::Account::LEN as u64,
        &spl_token::id(),
    );

    let init_token_account_instruction = token_instruction::initialize_account(
        &spl_token::id(),
        &token_account.pubkey(),
        &mint.pubkey(),
        &payer.pubkey(),
    )?;

    let mut create_and_init_tx = Transaction::new_with_payer(
        &[create_token_account_tx, init_token_account_instruction],
        Some(&payer.pubkey()),
    );
    create_and_init_tx.sign(&[&payer, &token_account], recent_blockhash);
    client.send_and_confirm_transaction(&create_and_init_tx)?;

    let mint_to_instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint.pubkey(),
        &token_account.pubkey(),
        &payer.pubkey(),
        &[],
        1000,
    )?;

    let mut mint_tx = Transaction::new_with_payer(
        &[mint_to_instruction],
        Some(&payer.pubkey()),
    );
    mint_tx.sign(&[&payer], recent_blockhash);
    client.send_and_confirm_transaction(&mint_tx)?;

    println!("Mint created and initialized, tokens minted.");
    Ok(())

}
