use std::{collections::HashMap, future::Future, str::FromStr, time::Duration};

use amico::{
    ai::tool::{Tool, ToolBuilder},
    resource::ResourceMut,
    runtime::storage::{Namespace, Storage},
};
use amico_core::{traits::EventSource, types::AgentEvent};
use amico_mods::{
    a2a::network::{A2aNetwork, dephy::DephyNetwork, error::NetworkError},
    runtime::storage::fs::FsStorage,
    web3::wallet::WalletResource,
};
use anyhow::anyhow;
use nostr::key::Keys;
use serde_json::{json, to_value};
use solana_sdk::pubkey::Pubkey;
use tokio::task::JoinHandle;

use super::events::A2aMessageReceived;

#[derive(Clone)]
pub struct A2aModule {
    network: A2aNetwork,
    storage: ResourceMut<FsStorage>,
}

impl A2aModule {
    pub fn new(wallet: WalletResource, storage: ResourceMut<FsStorage>) -> Self {
        // Setup wallet and keys
        let keys = Keys::generate();

        // Setup underlying network
        let dephy_network = DephyNetwork::new(keys, wallet);

        Self {
            network: A2aNetwork::new(dephy_network),
            storage,
        }
    }

    pub async fn connect(&self) -> Result<(), NetworkError> {
        self.network.connect().await
    }

    pub fn send_message_tool(&self) -> Tool {
        let network = self.network.clone().network;

        ToolBuilder::new()
            .name("send_a2a_message")
            .description("Send a message to another agent at the specified Solana address")
            .parameters(json!({
                "type": "object",
                "properties": {
                    "address": {
                        "type": "string",
                        "description": "The address to send message to"
                    },
                    "content": {
                        "type": "string",
                        "description": "The content of message"
                    }
                },
                "required": ["address", "content"]
            }))
            .build_async(move |args| {
                tracing::debug!("Calling send_a2a_message({})", args.clone());
                let network = network.clone();
                async move {
                    let address = args
                        .get("address")
                        .ok_or(anyhow!("Missing address parameter"))?;
                    let content = args
                        .get("content")
                        .ok_or(anyhow!("Missing content parameter"))?;

                    let pubkey = Pubkey::from_str(&address.to_string())?;

                    tracing::info!("Sending {} to {}...", content, address);

                    network.publish_dyn(pubkey, content.to_string()).await?;

                    tracing::info!("Message sent.");

                    Ok(json!({
                        "result": "Message sent successfully"
                    }))
                }
            })
    }

    pub fn contact_list_tool(&self) -> Tool {
        let storage = self.storage.get_ptr();
        ToolBuilder::new()
            .name("contact_list")
            .description("Get your contact address list of the Agent-to-agent network")
            .parameters(json!({}))
            .build_async(move |_| {
                let storage = storage.clone();

                async move {
                    let mut list = HashMap::new();
                    {
                        let mut fs_store = storage.lock().await;
                        let ns = fs_store.open_or_create("contact").unwrap();

                        for key in ns.keys().unwrap() {
                            let value = ns.get(&key).unwrap().unwrap().to_string().unwrap();
                            list.insert(key, value);
                        }
                    }
                    let value = to_value(list).unwrap();

                    Ok(value)
                }
            })
    }
}

impl EventSource for A2aModule {
    fn spawn<F, Fut>(&self, on_event: F) -> JoinHandle<anyhow::Result<()>>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let network = self.network.network.clone();
        tokio::spawn(async move {
            network
                .subscribe_dyn(Box::new(move |message| {
                    Box::pin(on_event(
                        AgentEvent::new("A2aMessageReceived", "A2aModule")
                            .with_content(A2aMessageReceived(message))
                            .unwrap(),
                    ))
                }))
                .await?;

            // Wait forever.
            tokio::time::sleep(Duration::MAX).await;

            Ok(())
        })
    }
}
