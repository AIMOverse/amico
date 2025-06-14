use std::str::FromStr;

use amico::{
    ai::tool::{Tool, ToolBuilder},
    environment::Sensor,
    resource::{IntoResource, Resource},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_client::client_error::ClientError;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signer::Signer};

use crate::web3::wallet::WalletResource;

use super::client::SolanaClientResource;

/// A sensor that gets the balance of a Solana account.
#[derive(Clone)]
pub struct BalanceSensor {
    client: SolanaClientResource,
    wallet: WalletResource,
}

impl BalanceSensor {
    /// Create a new balance sensor
    ///
    /// Arguments:
    ///    * `client` - The client resource.
    ///    * `wallet` - The wallet resource.
    ///
    /// Returns:
    ///    * `BalanceSensor` - The new balance sensor instance.
    pub fn new(client: SolanaClientResource, wallet: WalletResource) -> Self {
        Self { client, wallet }
    }

    /// Get a tool that can be used to get the balance of the wallet owned by the agent.
    ///
    /// Returns:
    ///    * `Tool` - The tool.
    ///
    /// **NOTE**: This is the temporary approach. Wait for `Environment` model to support sensors
    /// in the future release.
    pub fn agent_wallet_balance_tool(&self) -> Tool {
        // `Pubkey` implements the `Copy` trait, so we can just copy it
        let pubkey = self.wallet.get().solana().pubkey();
        let sensor = self.clone();

        ToolBuilder::new()
            .name("balance_sensor")
            .description("Get the balance of your own Solana account.")
            .parameters(serde_json::json!({}))
            .build_async(move |_| {
                // Clone the sensor and pubkey to move into the async block
                let sensor = sensor.clone();
                let pubkey = pubkey;

                // Return a boxed future that is both Send and Sync
                async move {
                    sensor
                        .sense(BalanceSensorArgs { pubkey })
                        .await
                        .map(|result| {
                            serde_json::json!({
                                "balance": result.lamports as f64 / LAMPORTS_PER_SOL as f64
                            })
                        })
                }
            })
    }

    /// Get a tool that can be used to get the balance of a Solana account.
    ///
    /// Returns:
    ///    * `Tool` - The tool.
    ///
    /// **NOTE**: This is the temporary approach. Wait for `Environment` model to support sensors
    /// in the future release.
    pub fn account_balance_tool(&self) -> Tool {
        let sensor = self.clone();
        ToolBuilder::new()
            .name("account_balance_tool")
            .description("Get the balance of a Solana account.")
            .parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "pubkey": {
                        "type": "string",
                        "description": "The Solana pubkey of the account."
                    }
                },
                "required": ["pubkey"],
            }))
            .build_async(move |args| {
                // Clone the sensor and pubkey to move into the async block
                let sensor = sensor.clone();
                let pubkey_arg = args["pubkey"].to_string();

                // Return a boxed future that is both Send and Sync
                async move {
                    // Parse the pubkey
                    let pubkey = Pubkey::from_str(&pubkey_arg)?;
                    sensor
                        .sense(BalanceSensorArgs { pubkey })
                        .await
                        .map(|result| {
                            serde_json::json!({
                                "balance": result.lamports as f64 / LAMPORTS_PER_SOL as f64
                            })
                        })
                }
            })
    }
}

impl IntoResource<BalanceSensor> for BalanceSensor {
    fn into_resource(self) -> Resource<BalanceSensor> {
        Resource::new("balance_sensor", self)
    }
}

/// Error type for balance sensor
#[derive(Debug, thiserror::Error)]
pub enum BalanceSensorError {
    #[error("Failed to get balance: {0}")]
    GetBalanceError(#[from] ClientError),
}

/// Arguments for the balance sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSensorArgs {
    pub pubkey: Pubkey,
}

/// Result of the balance sensor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSensorResult {
    pub lamports: u64,
}

#[async_trait]
impl Sensor for BalanceSensor {
    type Args = BalanceSensorArgs;
    type Output = BalanceSensorResult;

    /// Sense the balance of a Solana account
    ///
    /// Arguments:
    ///    * `args` - The arguments for the sensor.
    ///
    /// Returns:
    ///    * `Result<BalanceSensorResult, BalanceSensorError>` - The result of the sensor.
    async fn sense(&self, args: Self::Args) -> anyhow::Result<Self::Output> {
        let lamports = self
            .client
            .get()
            .rpc_client()
            .get_balance(&args.pubkey)
            .await?;
        Ok(BalanceSensorResult { lamports })
    }
}
