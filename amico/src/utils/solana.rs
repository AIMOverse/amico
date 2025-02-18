use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn get_balance_lamports(public_key: &str) -> Result<u64, Box<dyn std::error::Error>> {
    // Connect to the Solana cluster (e.g., mainnet-beta)
    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url);

    // Parse the public key from the provided string
    let pubkey = Pubkey::from_str(public_key)?;

    // Fetch the account balance in lamports
    let balance_lamports = client.get_balance(&pubkey)?;

    Ok(balance_lamports)
}

pub fn get_rpc_url() -> String {
    if let Ok(api_key) = std::env::var("HELIUS_API_KEY") {
        "https://mainnet.helius-rpc.com/?api-key=".to_string() + &api_key
    } else {
        "https://api.mainnet-beta.solana.com".to_string()
    }
}
