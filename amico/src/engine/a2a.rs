use std::{future::Future, str::FromStr};

use amico::ai::{
    errors::ToolCallError,
    tool::{Tool, ToolBuilder},
};
use amico_mods::{
    a2a::network::{dephy::DephyNetwork, error::NetworkError, A2aNetwork},
    web3::wallet::WalletResource,
};
use nostr::key::Keys;
use serde_json::json;
use solana_sdk::pubkey::Pubkey;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct A2aModule {
    network: A2aNetwork,
}

impl A2aModule {
    pub fn new(wallet: WalletResource) -> Self {
        // Setup wallet and keys
        let keys = Keys::generate();

        // Setup underlying network
        let dephy_network = DephyNetwork::new(keys, wallet);

        Self {
            network: A2aNetwork::new(dephy_network),
        }
    }

    pub async fn connect(&self) -> Result<(), NetworkError> {
        self.network.connect().await
    }

    pub fn spawn_event_source<F, Fut>(&self, on_event: F) -> JoinHandle<()>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let network = self.network.clone().network;
        let on_event = Box::pin(on_event);
        tokio::spawn(async move {
            network
                .subscribe_dyn(Box::new(move |message| Box::pin(on_event(message))))
                .await
                .inspect_err(|err| tracing::error!("{}", err))
                .unwrap();
        })
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
                    let address = args.get("address").ok_or(ToolCallError::InvalidParam {
                        name: "address".to_string(),
                        value: json!({}),
                        reason: "Missing".to_string(),
                    })?;
                    let content = args.get("content").ok_or(ToolCallError::InvalidParam {
                        name: "content".to_string(),
                        value: json!({}),
                        reason: "Missing".to_string(),
                    })?;

                    let pubkey = Pubkey::from_str(&content.to_string()).map_err(|err| {
                        ToolCallError::InvalidParam {
                            name: "address".to_string(),
                            value: address.clone(),
                            reason: err.to_string(),
                        }
                    })?;

                    tracing::info!("Sending {} to {}...", content, address);

                    network
                        .publish_dyn(pubkey, content.to_string())
                        .await
                        .map_err(|err| ToolCallError::ExecutionError {
                            tool_name: "send_a2a_message".to_string(),
                            params: args,
                            reason: err.to_string(),
                        })?;

                    tracing::info!("Message sent.");

                    Ok(json!({
                        "result": "Message sent successfully"
                    }))
                }
            })
    }
}
