mod programs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
    };
    use solana_client::rpc_client::RpcClient;
    use solana_program::system_instruction::transfer;
    use std::io::{self, BufRead};
    use std::str::FromStr;
    use bs58;
    // use solana_sdk::{signer::Signer, system_program, signature::read_keypair_file};
    use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};
    use solana_program::system_program;


    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();

        // Print the public key
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());

        // Print instructions for saving the wallet
        println!("\nTo save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");

        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();

        let wallet = bs58::decode(base58).into_vec().unwrap();

        println!("Your wallet file is:");
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");

        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let base58 = bs58::encode(wallet).into_string();

        println!("Your private key is:");
        println!("{:?}", base58);
    }

    #[test]
    fn claim_airdrop() {
        let keypair = match read_keypair_file("dev-wallet.json") {
            Ok(keypair) => keypair,
            Err(err) => {
                println!("Error reading wallet file: {}", err);
                return;
            }
        };

        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    signature.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = match read_keypair_file("dev-wallet.json") {
            Ok(keypair) => keypair,
            Err(err) => {
                println!("Error reading wallet file: {}", err);
                return;
            }
        };

        let to_pubkey = Pubkey::from_str("5sus6wj7rbUSEhoEZ364ESszPNee11JNkcuH1mW93WGG").unwrap();

        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn test_complete_instruction() {
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL); 

        // Define the signer (your wallet)
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        // Define the PDA for the prereq account
        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]); 

        // Define the instruction arguments
        let args = CompleteArgs { 
            github: b"zackmendel".to_vec() };            

        // Get the latest blockhash
        let blockhash = rpc_client .get_latest_blockhash() .expect("Failed to get recent blockhash");

        // Create the transaction
        let transaction = Turbin3PrereqProgram::complete( 
                &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()), &[&signer], 
                blockhash );


        // Send and confirm the transaction
        let signature = rpc_client .send_and_confirm_transaction(&transaction) .expect("Failed to send transaction"); 
        // Print our transaction out 
        println!("Success! Check out your TX here:
https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    } 
}