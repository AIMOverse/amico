use std::sync::Arc;

use amico::a2a::network::NetworkDyn;
use solana_sdk::pubkey::Pubkey;

use super::error::NetworkError;

#[derive(Clone)]
pub struct A2aNetwork {
    pub network:
        Arc<dyn NetworkDyn<Message = String, Address = Pubkey, Error = NetworkError> + Send + Sync>,
}

impl A2aNetwork {
    /// Create a new A2aNetwork instance
    pub fn new<N>(network: N) -> Self
    where
        N: NetworkDyn<Message = String, Address = Pubkey, Error = NetworkError>
            + Send
            + Sync
            + 'static,
    {
        Self {
            network: Arc::new(network),
        }
    }

    pub async fn connect(&self) -> Result<(), NetworkError> {
        self.network.connect_dyn().await
    }
}

#[cfg(test)]
mod tests {
    use amico::resource::Resource;
    use nostr::key::Keys;

    use crate::{a2a::network::dephy::DephyNetwork, web3::wallet::Wallet};

    use super::*;

    #[test]
    fn simulate_a2a_use() {
        // Setup wallet and keys
        let keys = Keys::generate();
        let wallet = Wallet::new().unwrap();
        let wallet_resource = Resource::new("Test wallet", wallet);

        // Setup underlying network
        let dephy_network = DephyNetwork::new(keys, wallet_resource);

        // Setup A2A network
        let _network_resource = Resource::new("Test network", A2aNetwork::new(dephy_network));
    }
}
