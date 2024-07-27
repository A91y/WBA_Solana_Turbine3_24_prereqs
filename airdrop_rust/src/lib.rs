#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        signature::{read_keypair_file, Keypair, Signer},
        transaction::Transaction,
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;
    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string() // 8ZUQ1vMqgpmd4jvaQLucKtT5nbvEYcyyQAvrUKkZ1oaB
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn airdop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Connect to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
                // https://explorer.solana.com/tx/iF8ugwXJzNEooHQtbqSvGE6B9rGt77DWfhkKWSraxUFtEc8BVsZe6av1pvHpdEbuAASB4HDwxm1BnxgU1PSdv66?cluster=devnet
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        }
    }
    #[test]
    fn transfer_0_01_sol() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        // Define our WBA public key
        let to_pubkey = Pubkey::from_str("HX1TPzh21wV1SFaENyu2YuAWzLfTL7ADjEtcNNdTCXiW").unwrap();
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        // create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
        // https://explorer.solana.com/tx/52MiJf5jBbqMb1iF6XvdhA36HCJb2rbeZgsey2ppiPfRE3Nrq7hkoa4hQczgJSVTcv6X3tgFNQUasMV2LJmA6RTd/?cluster=devnet
    }
    #[test]
    fn transfer_sol() {}
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
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
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
        // 3L2m6Rxv9maD2Cv92nEF3MssU1v5Js9CwnKyE6jUB3MHu4T1BKC7H72EXG52fy68vqaTo8kTpA8brnr8Y8DhgMHM
    }
}
