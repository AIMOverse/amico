use amico::environment::Sensor;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;

use super::resources::SolanaClientResource;

/// A sensor that gets the balance of a Solana account.
#[derive(Clone)]
pub struct BalanceSensor {
    client: SolanaClientResource,
}

impl BalanceSensor {
    /// Create a new balance sensor
    ///
    /// Arguments:
    ///    * `client` - The client resource.
    ///
    /// Returns:
    ///    * `BalanceSensor` - The new balance sensor instance.
    pub fn new(client: SolanaClientResource) -> Self {
        Self { client }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BalanceSensorError {
    #[error("Failed to get balance: {0}")]
    GetBalanceError(#[from] ClientError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSensorArgs {
    pub pubkey: Pubkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSensorResult {
    pub lamports: u64,
}

#[async_trait]
impl Sensor for BalanceSensor {
    type Args = BalanceSensorArgs;
    type Output = BalanceSensorResult;
    type Error = BalanceSensorError;

    /// Sense the balance of a Solana account
    ///
    /// Arguments:
    ///    * `args` - The arguments for the sensor.
    ///
    /// Returns:
    ///    * `Result<BalanceSensorResult, BalanceSensorError>` - The result of the sensor.
    async fn sense(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let lamports = self
            .client
            .value()
            .rpc_client()
            .get_balance(&args.pubkey)
            .await?;
        Ok(BalanceSensorResult { lamports })
    }
}
