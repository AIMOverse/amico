use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

pub fn swap(
    _client: &RpcClient,
    _keypair: &Keypair,
    _input_mint: &str,
    _output_mint: &str,
    _amount: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Trading {} for {}", _input_mint, _output_mint);
    tracing::warn!("Trading on solana not implemented yet.");
    Ok(())
}
