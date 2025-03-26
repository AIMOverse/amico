use amico::resource::Resource;
use solana_client::nonblocking::rpc_client::RpcClient;

/// Solana client resource uses Solana's non-blocking client.
pub type SolanaClientResource = Resource<RpcClient>;
