use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;

/// Swap tokens on Raydium
///
/// Arguments:
///    * `client` - The client resource.
///    * `keypair` - The wallet resource.
///    * `input_mint` - The Solana pubkey of input token mint, 'sol' for native token.
///    * `output_mint` - The Solana pubkey of output token mint, 'sol' for native token.
///    * `amount` - The amount of token to trade.
///
/// Returns:
///    * `Result<(), Box<dyn std::error::Error>>` - The result of the swap.
///
/// **TODO**: Implement the swap function.
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
