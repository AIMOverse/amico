use std::{future::Future, pin::Pin};

use amico::{a2a::network::Network, resource::Resource};
use nostr::{
    event::{EventBuilder, Kind, Tag},
    types::{Filter, SingleLetterTag, Timestamp},
};
use nostr_sdk::{Client, RelayPoolNotification};
use solana_sdk::signer::Signer;

use crate::a2a::crypto;
use crate::web3::wallet::Wallet;

use super::{error::NetworkError, interface::A2aNetwork};

#[derive(Clone)]
pub struct DephyNetwork {
    client: nostr_sdk::Client,
    wallet: Resource<Wallet>,
}

const MENTION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::P);
const SESSION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::S);
const SESSION_ID: &str = "amico_dephy_session";

impl DephyNetwork {
    /// Create a new DephyNetwork instance
    pub fn new(keys: nostr::Keys, wallet: Resource<Wallet>) -> Self {
        let client_opts = nostr_sdk::Options::default();
        let client = Client::builder().opts(client_opts).signer(keys).build();
        Self { client, wallet }
    }
}

impl Network for DephyNetwork {
    type Message = String;
    type Address = solana_sdk::pubkey::Pubkey;
    type Error = NetworkError;

    async fn connect(&self) -> Result<(), Self::Error> {
        self.client
            .add_relay("wss://canary-relay.dephy.dev")
            .await?;
        self.client.connect().await;

        Ok(())
    }

    async fn publish(
        &self,
        address: Self::Address,
        message: Self::Message,
    ) -> Result<(), Self::Error> {
        let cipher_text = crypto::encrypt_message(&message, &address)?;
        // Tag with session and receiver Solana pubkey
        let event = EventBuilder::new(Kind::Custom(1573), cipher_text).tags([
            Tag::parse([SESSION_TAG.to_string(), SESSION_ID.to_string()])?,
            Tag::parse([MENTION_TAG.to_string(), address.to_string()])?,
        ]);

        self.client.send_event_builder(event).await?;

        let from_address = self.wallet.value().solana().pubkey().to_string();
        tracing::info!("Published cipher text from {from_address} to {address}: {message}");

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
        let client = self.client.clone();
        let pubkey = self.wallet.value().solana().pubkey().to_string();
        let wallet = self.wallet.clone();

        let filter = Filter::new()
            .kind(Kind::Custom(1573))
            .since(Timestamp::now())
            .custom_tag(SESSION_TAG, [SESSION_ID])
            .custom_tag(MENTION_TAG, [pubkey.as_str()]);

        // Subscribe to the filter
        client.subscribe(vec![filter], None).await?;

        // spawn a task to handle notifications
        tokio::spawn(async move {
            if let Err(e) = client
                .handle_notifications(|notification| async {
                    if let RelayPoolNotification::Event { event, .. } = notification {
                        // Just log error messages. Errors are not fatal here.
                        tracing::info!("Received cipher text {}", event.content);
                        let keypair = wallet.value().solana();
                        // Decrypt
                        if let Ok(plaintext) = crypto::decrypt_message(&event.content, keypair) {
                            tracing::info!("Decrypted message {}", plaintext);
                            on_message(plaintext).await;
                        } else {
                            tracing::info!("Failed to decrypt message");
                        }
                    }
                    Ok(false) // Keep listening
                })
                .await
            {
                tracing::error!("Failed to handle notifications: {}", e);
            }
        });

        Ok(())
    }
}

// Convert Dephy Network to A2A Network
impl From<DephyNetwork> for A2aNetwork {
    fn from(value: DephyNetwork) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use nostr::key::Keys;
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration};

    use super::*;

    /// Simulate an actual message pubsub process.
    ///
    /// This test relies on the dephy relay and might be unstable
    /// to execute, so it's not suitable for integration tests.
    ///
    /// You have to run the test manually.
    #[ignore]
    #[tokio::test]
    async fn test_network_pubsub() {
        // Setup two network clients
        let publisher = Resource::new("wallet1".to_string(), Wallet::new().unwrap());
        let subscriber = Resource::new("wallet2".to_string(), Wallet::new().unwrap());

        // Create two DephyNetwork instances
        let publisher_network = DephyNetwork::new(Keys::generate(), publisher.clone());
        let subscriber_network = DephyNetwork::new(Keys::generate(), subscriber.clone());

        // Connect both networks
        publisher_network.connect().await.unwrap();
        subscriber_network.connect().await.unwrap();

        // Create a channel
        let (tx, mut rx) = mpsc::channel(1);

        // Spawn listeners
        subscriber_network
            .subscribe(Box::new(move |message| {
                let tx = tx.clone();
                Box::pin(async move {
                    tracing::info!("Received message: {}", message);
                    tx.send(message).await.unwrap();
                })
            }))
            .await
            .unwrap();

        // Publish a message
        publisher_network
            .publish(subscriber.value().solana().pubkey(), "test".to_string())
            .await
            .unwrap();

        // Wait for the message with timeout
        let received_message = timeout(Duration::from_secs(10), rx.recv())
            .await
            .expect("Timeout waiting for message")
            .expect("Channel closed");
        assert_eq!(received_message, "test");

        // Send another message
        publisher_network
            .publish(subscriber.value().solana().pubkey(), "test2".to_string())
            .await
            .unwrap();

        // Wait for the message with timeout
        let received_message = timeout(Duration::from_secs(10), rx.recv())
            .await
            .expect("Timeout waiting for message")
            .expect("Channel closed");
        assert_eq!(received_message, "test2");
    }
}
