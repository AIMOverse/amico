use amico::environment::Effector;
use async_trait::async_trait;

use crate::web3::wallet::WalletResource;

use super::{resources::SolanaClientResource, utils::swap::swap};

#[derive(Clone)]
pub struct TradeEffector {
    client: SolanaClientResource,
    wallet: WalletResource,
}

impl TradeEffector {
    pub fn new(client: SolanaClientResource, wallet: WalletResource) -> Self {
        Self { client, wallet }
    }
}

pub struct TradeEffectorArgs {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String,
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
            &args.amount,
        )
        .map_err(|_| TradeError::ClientError)
    }
}
