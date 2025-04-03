pub fn solana_rpc_url(cluster: &str) -> String {
    if let Ok(api_key) = std::env::var("HELIUS_API_KEY") {
        format!("https://{}.helius-rpc.com/?api-key={}", cluster, api_key)
    } else {
        format!("https://api.{}.solana.com", cluster)
    }
}
