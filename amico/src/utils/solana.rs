use solana_client::rpc_client::RpcClient;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use std::str::FromStr;

pub fn get_balance_in_sol(public_key: &str) -> Result<f64, Box<dyn std::error::Error>> {
    // Connect to the Solana cluster (e.g., mainnet-beta)
    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url);

    // Parse the public key from the provided string
    let pubkey = Pubkey::from_str(public_key)?;

    // Fetch the account balance in lamports
    let balance_lamports = client.get_balance(&pubkey)?;

    // Convert lamports to SOL
    let balance_sol = balance_lamports as f64 / LAMPORTS_PER_SOL as f64;

    Ok(balance_sol)
}

pub fn get_rpc_url() -> String {
    if let Ok(api_key) = std::env::var("HELIUS_API_KEY") {
        "https://devnet.helius-rpc.com/?api-key=".to_string() + &api_key
    } else {
        "https://api.devnet.solana.com".to_string()
    }
}
