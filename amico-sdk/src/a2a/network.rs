use async_trait::async_trait;

/// A trait for agent-to-agent (A2A) network communication.
///
/// This trait defines the network model (pubsub model)
/// and the interface for A2A network communication.
#[async_trait]
pub trait Network {
    type Message;
    type Address;
    type Error;

    /// Connect to the network.
    async fn connect(&self) -> Result<(), Self::Error>;

    /// Publish a message to a specific receiver.
    async fn publish(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error>;

    /// Subscribe to the network.
    async fn subscribe(&self) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

    use super::*;

    #[derive(thiserror::Error, Debug)]
    pub enum TestError {}

    struct TestNetwork;

    #[async_trait]
    impl Network for TestNetwork {
        type Message = String;
        type Address = Pubkey;
        type Error = TestError;

        async fn connect(&self) -> Result<(), Self::Error> {
            println!("Connecting to network");
            Ok(())
        }

        async fn publish(
            &self,
            receiver: Self::Address,
            message: Self::Message,
        ) -> Result<(), Self::Error> {
            println!("Publishing message to {receiver}: {message}");
            Ok(())
        }

        async fn subscribe(&self) -> Result<(), Self::Error> {
            println!("Subscribing to network");
            Ok(())
        }
    }

    #[test]
    fn test_network_dyn_compatible() {
        let _: Box<dyn Network<Message = String, Address = Pubkey, Error = TestError>> =
            Box::new(TestNetwork);
    }

    #[tokio::test]
    async fn test_network_methods() {
        let network: Box<dyn Network<Message = String, Address = Pubkey, Error = TestError>> =
            Box::new(TestNetwork);
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        network.connect().await.unwrap();
        network.publish(pubkey, "test".to_string()).await.unwrap();
        network.subscribe().await.unwrap();
    }
}
