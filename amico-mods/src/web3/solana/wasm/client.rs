use amico::resource::Resource;

pub type RpcClient = wasm_client_solana::SolanaRpcClient;

pub struct SolanaClient(RpcClient);

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        Self(RpcClient::new(rpc_url))
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.0
    }
}

/// Solana client resource uses Solana's non-blocking client.
pub type SolanaClientResource = Resource<SolanaClient>;
