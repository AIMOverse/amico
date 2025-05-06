use std::pin::Pin;

use async_trait::async_trait;

/// The message handler clusure type
pub type MessageHandler<M> =
    Box<dyn Fn(M) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static>;

/// A trait for agent-to-agent (A2A) network communication.
///
/// This trait defines the network model (pubsub model)
/// and the interface for A2A network communication.
pub trait Network {
    type Message;
    type Address;
    type Error;

    /// Connect to the network.
    fn connect(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Publish a message to a specific receiver.
    fn publish(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Spawn an event handler and subscribe to the network.
    fn subscribe(
        &self,
        on_message: MessageHandler<Self::Message>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

// Dynamic and local traits

#[async_trait]
pub trait NetworkDyn {
    type Message;
    type Address;
    type Error;

    async fn connect_dyn(&self) -> Result<(), Self::Error>;
    async fn publish_dyn(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error>;
    async fn subscribe_dyn(
        &self,
        on_message: MessageHandler<Self::Message>,
    ) -> Result<(), Self::Error>;
}

#[async_trait(?Send)]
pub trait NetworkLocal {
    type Message;
    type Address;
    type Error;

    async fn connect_local(&self) -> Result<(), Self::Error>;
    async fn publish_local(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error>;
    async fn subscribe_local(
        &self,
        on_message: MessageHandler<Self::Message>,
    ) -> Result<(), Self::Error>;
}

// Implement dyn and local traits for all types

#[async_trait]
impl<T: Network + Sync> NetworkDyn for T
where
    T::Address: Send,
    T::Message: Send,
{
    type Address = T::Address;
    type Message = T::Message;
    type Error = T::Error;

    async fn connect_dyn(&self) -> Result<(), Self::Error> {
        self.connect().await
    }
    async fn publish_dyn(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error> {
        self.publish(receiver, message).await
    }

    async fn subscribe_dyn(
        &self,
        on_message: MessageHandler<Self::Message>,
    ) -> Result<(), Self::Error> {
        self.subscribe(on_message).await
    }
}

#[async_trait(?Send)]
impl<T: Network> NetworkLocal for T {
    type Address = T::Address;
    type Message = T::Message;
    type Error = T::Error;

    async fn connect_local(&self) -> Result<(), Self::Error> {
        self.connect().await
    }
    async fn publish_local(
        &self,
        receiver: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error> {
        self.publish(receiver, message).await
    }

    async fn subscribe_local(
        &self,
        on_message: MessageHandler<Self::Message>,
    ) -> Result<(), Self::Error> {
        self.subscribe(on_message).await
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

    use super::*;

    #[derive(thiserror::Error, Debug)]
    pub enum TestError {}

    struct TestNetwork;

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

        async fn subscribe(
            &self,
            on_message: Box<
                dyn Fn(Self::Message) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
                    + Send
                    + Sync
                    + 'static,
            >,
        ) -> Result<(), Self::Error> {
            println!("Subscribing to network");
            on_message("test".to_string()).await;
            Ok(())
        }
    }

    #[test]
    fn test_network_dyn_compatible() {
        let _: Box<dyn NetworkDyn<Message = String, Address = Pubkey, Error = TestError>> =
            Box::new(TestNetwork);
        let _: Box<dyn NetworkLocal<Message = String, Address = Pubkey, Error = TestError>> =
            Box::new(TestNetwork);
    }

    #[tokio::test]
    async fn test_network_methods() {
        let network = TestNetwork;
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        network.connect().await.unwrap();
        network.publish(pubkey, "test".to_string()).await.unwrap();
        network
            .subscribe(Box::new(|_message| Box::pin(async {})))
            .await
            .unwrap();
    }
}
