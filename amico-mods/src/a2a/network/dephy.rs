use std::sync::Arc;

use amico::{a2a::network::Network, resource::Resource};
use async_trait::async_trait;
use nostr::{
    event::{EventBuilder, Kind, Tag},
    types::{Filter, SingleLetterTag, Timestamp},
};
use nostr_sdk::{Client, RelayPoolNotification};
use solana_sdk::signer::Signer;

use crate::a2a::crypto;
use crate::web3::wallet::Wallet;

pub struct DephyNetwork {
    client: nostr_sdk::Client,
    wallet: Resource<Wallet>,
}

const MENTION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::P);
const SESSION_TAG: SingleLetterTag = SingleLetterTag::lowercase(nostr::Alphabet::S);
const SESSION_ID: &str = "amico_dephy_session";

#[derive(thiserror::Error, Debug)]
pub enum DephyError {
    #[error("NoStr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::a2a::crypto::CryptoError),

    #[error("NoStr tag error: {0}")]
    TagError(#[from] nostr::event::tag::Error),
}

impl DephyNetwork {
    pub fn new(keys: nostr::Keys, wallet: Resource<Wallet>) -> Self {
        let client_opts = nostr_sdk::Options::default();
        let client = Client::builder().opts(client_opts).signer(keys).build();
        Self { client, wallet }
    }

    pub async fn spawn_listener(
        self_arc: Arc<Self>,
        channel: std::sync::mpsc::Sender<String>,
    ) -> Result<(), DephyError> {
        let client = self_arc.client.clone();
        let pubkey = self_arc
            .wallet
            .value()
            .solana_keypair()
            .pubkey()
            .to_string();

        let filter = Filter::new()
            .kind(Kind::Custom(1573))
            .since(Timestamp::now())
            .custom_tag(SESSION_TAG, [SESSION_ID])
            .custom_tag(MENTION_TAG, [pubkey.as_str()]);

        // Subscribe to the filter
        client.subscribe(vec![filter], None).await?;

        // Handle notifications
        client
            .handle_notifications(|notification| async {
                match notification {
                    RelayPoolNotification::Event { event, .. } => {
                        tracing::info!("Received cipher text {}", event.content);
                        let keypair = self_arc.wallet.value().solana_keypair();
                        // Decrypt
                        if let Ok(plaintext) = crypto::decrypt_message(&event.content, keypair) {
                            tracing::info!("Decrypted message {}", plaintext);
                            let _ = channel.send(plaintext).inspect_err(|err| {
                                tracing::error!("Failed to send message to channel: {}", err)
                            });
                        } else {
                            tracing::info!("Failed to decrypt message");
                        }
                    }
                    _ => {} // Ignore other notification types
                }
                Ok(false) // Keep listening
            })
            .await?;

        Ok(())
    }
}

#[async_trait]
impl Network for DephyNetwork {
    type Message = String;
    type Address = solana_sdk::pubkey::Pubkey;
    type Error = DephyError;

    async fn connect(&self) -> Result<(), Self::Error> {
        self.client.add_relay("wss://dev-relay.dephy.dev").await?;
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

        let from_address = self.wallet.value().solana_keypair().pubkey().to_string();
        tracing::info!("Published cipher text from {from_address} to {address}: {message}");

        Ok(())
    }

    async fn on_message(&self, message: Self::Message) {
        tracing::info!("Received message: {message}");
    }
}

#[cfg(test)]
mod tests {
    use nostr::key::Keys;

    use super::*;

    // This test should be executed by hand. Ignored by default.
    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
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
        let (tx, rx) = std::sync::mpsc::channel();

        // Spawn listeners
        let subscriber_arc = Arc::new(subscriber_network);
        tokio::spawn(async move {
            DephyNetwork::spawn_listener(subscriber_arc, tx)
                .await
                .unwrap();
        });

        // Publish a message
        publisher_network
            .publish(
                subscriber.value().solana_keypair().pubkey(),
                "test".to_string(),
            )
            .await
            .unwrap();

        // Wait for the message
        let received_message = rx.recv().unwrap();
        assert_eq!(received_message, "test");
    }
}
