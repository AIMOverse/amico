use std::sync::Arc;

use amico::a2a::network::Network;
use solana_sdk::pubkey::Pubkey;

use super::error::NetworkError;

pub struct A2aNetwork {
    pub network: Arc<dyn Network<Message = String, Address = Pubkey, Error = NetworkError>>,
}

impl A2aNetwork {
    /// Create a new A2aNetwork instance
    pub fn new<N>(network: N) -> Self
    where
        N: Network<Message = String, Address = Pubkey, Error = NetworkError> + 'static,
    {
        Self {
            network: Arc::new(network),
        }
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
        let wallet_resource = Resource::new("Test wallet".to_string(), wallet);

        // Setup underlying network
        let dephy_network = DephyNetwork::new(keys, wallet_resource);

        // Setup A2A network
        let _network_resource =
            Resource::new("Test network".to_string(), A2aNetwork::new(dephy_network));
    }
}
