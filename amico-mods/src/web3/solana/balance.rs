use amico::environment::Sensor;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use solana_client::client_error::ClientError;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

use super::resources::ClientResource;

/// A sensor that gets the balance of a Solana account.
#[derive(Clone)]
pub struct BalanceSensor {
    client: Arc<ClientResource>,
}

impl BalanceSensor {
    pub fn new(client: Arc<ClientResource>) -> Self {
        Self { client }
    }

    async fn get_lamports(&self, pubkey: &Pubkey) -> Result<u64, ClientError> {
        self.client.value().get_balance(pubkey).await
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
    type Result = BalanceSensorResult;
    type Error = BalanceSensorError;

    async fn sense(&self, args: Self::Args) -> Result<Self::Result, Self::Error> {
        let lamports = self.get_lamports(&args.pubkey).await?;
        Ok(BalanceSensorResult { lamports })
    }
}
