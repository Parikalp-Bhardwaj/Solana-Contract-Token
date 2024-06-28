use solana_client::{ rpc_client::RpcClient};

use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::{read_keypair_file, Keypair, Signer}, sysvar::recent_blockhashes, transaction::Transaction};
use std::{fs,path::PathBuf};
use std::error::Error;
use spl_token::instruction as token_instruction;
use tokio;

use solana_program::{entrypoint};

pub fn keypair_path_payer() -> Result<Keypair, Box<dyn Error>> {
    let mut keypair_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    keypair_path.extend(["/Users/parikalpbhardwaj/Coding_Zone/Solana-Blockahian/solana-deploy-token", "contract", "program", "solana_deploy_token-keypair.json"]);

    let program_keypair = read_keypair_file(&keypair_path).map_err(|e| format!("Failed to read program keypair file: {}", e))?;

 
    // let program_id = program_keypair.pubkey();
    println!("Program ID: {:?}", program_keypair.pubkey());

    // println!("Program ID: {}", program_id);

    let mut payer_keypair_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    payer_keypair_path.extend(["/Users/parikalpbhardwaj/.config/solana", "id.json"]); // Use a persistent keypair

    let payer = read_keypair_file(&payer_keypair_path).map_err(|e| format!("Failed to read payer keypair file: {}", e))?;

    Ok(payer)
}


pub fn check_balance(client: &RpcClient, payer: Pubkey)-> Result<u64, Box<dyn Error>>{
    let balance = client.get_balance(&payer)?;
    Ok(balance)
}


async fn approve_tokens(
    client: &RpcClient, 
    payer: &Keypair, 
    owner_account: &Pubkey, 
    delegate_account: &Pubkey, 
    amount: u64
) -> Result<(), Box<dyn Error>> {
    let approve_instruction = token_instruction::approve(
        &spl_token::id(),
        owner_account,
        delegate_account,
        &payer.pubkey(),
        &[&payer.pubkey()],
        amount,
    )?;

    let recent_blockhash = client.get_latest_blockhash()?;
    let mut transaction = Transaction::new_with_payer(
        &[approve_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    client.send_and_confirm_transaction(&transaction)?;
    Ok(())
}

async fn transfer_from(client: &RpcClient,delegate: &Keypair, 
                source_account: &Pubkey, 
                destination_account: &Pubkey, 
                owner: &Pubkey, 
                amount: u64,
            ) -> Result<(), Box<dyn Error>>{
        
        let transfer_from_instruction = token_instruction::transfer(
            &spl_token::id(), source_account, destination_account,
            owner, &[&delegate.pubkey()], amount)?;

        let recent_blockhashes = client.get_latest_blockhash()?;

        let mut transaction = Transaction::new_with_payer(
            &[transfer_from_instruction],
            Some(&delegate.pubkey()),
        );

        transaction.sign(&[delegate], recent_blockhashes);
        client.send_and_confirm_transaction(&transaction)?;
        

        Ok(())
}


async fn send_transaction(
    rpc_url: &str,
    payer: &Keypair,
    instruction: Instruction,
) -> Result<(), Box<dyn Error>> {
    let client = RpcClient::new(String::from(rpc_url));
    let recent_blockhash = client.get_latest_blockhash()?;

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[payer], recent_blockhash);
    client.send_and_confirm_transaction(&transaction)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::system_instruction;


    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_transfer_sols() -> Result<(), Box<dyn Error>> {
        let payer = keypair_path_payer().unwrap(); 
        let recipient = Pubkey::new_unique();

        let transfer_instruction = system_instruction::transfer(
            &payer.pubkey(),
            &recipient,
            1_000_000, 
        );

        let rpc_url = "http://127.0.0.1:8899"; 
        send_transaction(rpc_url, &payer, transfer_instruction).await?;

        let client = RpcClient::new(String::from(rpc_url));
        let balance = client.get_balance(&recipient)?;
        dbg!(balance);
        assert_eq!(balance, 1_000_000);

        Ok(())
    }
}