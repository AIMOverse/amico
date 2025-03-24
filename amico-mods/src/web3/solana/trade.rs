use std::sync::Arc;

use amico::environment::Effector;
use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;

use crate::web3::wallet::WalletResource;

use super::{resources::ClientResource, utils::swap::swap};

pub struct TradeEffector {
    client: Arc<ClientResource>,
    wallet: Arc<WalletResource>,
}

impl TradeEffector {
    pub fn new(client: Arc<ClientResource>, wallet: Arc<WalletResource>) -> Self {
        Self { client, wallet }
    }
}

pub struct TradeEffectorArgs {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub amount: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum TradeError {
    #[error("Client error")]
    ClientError,
}

#[async_trait]
impl Effector for TradeEffector {
    type Args = TradeEffectorArgs;
    type Output = ();
    type Error = TradeError;

    async fn effect(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        swap(
            self.client.value(),
            self.wallet.value().solana_keypair(),
            &args.input_mint,
            &args.output_mint,
            args.amount,
        )
        .map_err(|_| TradeError::ClientError)
    }
}
