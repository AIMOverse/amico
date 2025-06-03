use amico::resource::{IntoResource, Resource};

pub type RpcClient = solana_client::nonblocking::rpc_client::RpcClient;

pub struct SolanaClient(RpcClient);

impl SolanaClient {
    pub fn new(rpc_url: &str) -> Self {
        Self(RpcClient::new(rpc_url.to_string()))
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.0
    }
}

impl IntoResource<SolanaClient> for SolanaClient {
    fn into_resource(self) -> Resource<SolanaClient> {
        Resource::new("solana-client", self)
    }
}

pub type SolanaClientResource = Resource<SolanaClient>;
