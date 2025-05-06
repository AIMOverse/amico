use amico::resource::Resource;

pub type RpcClient = solana_client::nonblocking::rpc_client::RpcClient;

pub struct SolanaClient(RpcClient);

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        return Self(RpcClient::new(rpc_url.to_string()));
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.0
    }
}

/// Solana client resource uses Solana's non-blocking client.
pub type SolanaClientResource = Resource<SolanaClient>;
