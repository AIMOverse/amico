use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};

pub fn get_balance_in_sol(public_key: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Connect to the Solana cluster (e.g., mainnet-beta)
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url);

    // Parse the public key from the provided string
    let pubkey = Pubkey::from_str(public_key)?;

    // Fetch the account balance in lamports
    let balance_lamports = client.get_balance(&pubkey)?;

    // Convert lamports to SOL
    let balance_sol = balance_lamports as f64 / LAMPORTS_PER_SOL as f64;

    Ok(balance_sol)
}