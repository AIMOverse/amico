use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub fn swap(
    _client: &RpcClient,
    _keypair: &Keypair,
    _input_mint: &Pubkey,
    _output_mint: &Pubkey,
    _amount: u64,
) -> Result<(), ()> {
    todo!();
}
