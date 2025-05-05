use amico::{
    ai::{
        errors::ToolCallError,
        tool::{Tool, ToolBuilder},
    },
    environment::Effector,
};
use async_trait::async_trait;

use crate::web3::wallet::WalletResource;

use super::{resources::SolanaClientResource, utils::swap::swap};

/// A effector that trades solana tokens on Raydium
#[derive(Clone)]
pub struct TradeEffector {
    client: SolanaClientResource,
    wallet: WalletResource,
}

impl TradeEffector {
    /// Create a new trade effector
    ///
    /// Arguments:
    ///    * `client` - The client resource.
    ///    * `wallet` - The wallet resource.
    ///
    /// Returns:
    ///    * `TradeEffector` - The new trade effector instance.
    pub fn new(client: SolanaClientResource, wallet: WalletResource) -> Self {
        Self { client, wallet }
    }

    /// Get a tool that can be used to trade solana tokens on Raydium
    ///
    /// Arguments:
    ///    * `effector` - The trade effector.
    ///
    /// Returns:
    ///    * `Tool` - The trade tool.
    ///
    /// **NOTE**: This is the temporary approach. Wait for `Environment` model to support effector
    /// in the future release.
    pub fn tool(&self) -> Tool {
        let effector = self.clone();
        ToolBuilder::new()
            .name("trade_solana_token")
            .description("Trade solana tokens on Raydium")
            .parameters(serde_json::json!({
                "type": "object",
                "properties": {
                    "input_mint": {
                        "type": "string",
                        "description": "The Solana pubkey of input token mint, 'sol' for native token."
                    },
                    "output_mint": {
                        "type": "string",
                        "description": "The Solana pubkey of output token mint, 'sol' for native token."
                    },
                    "amount": {
                        "type": "string",
                        "description": "The amount of token to trade."
                    }
                },
                "required": ["input_mint", "output_mint", "amount"],
            }))
            .build_async(move |args| {
                let effector = effector.clone();
                let effector_args = TradeEffectorArgs {
                    input_mint: args["input_mint"].to_string(),
                    output_mint: args["output_mint"].to_string(),
                    amount: args["amount"].to_string(),
                };

                async move {
                    effector
                        .effect(effector_args)
                        .await
                        .map(|_| serde_json::json!({"status": "success"}))
                        .map_err(|err| ToolCallError::ExecutionError {
                            tool_name: "trade_solana_token".to_string(),
                            params: args,
                            reason: err.to_string(),
                        })
                }
            })
    }
}

/// Arguments for the trade effector
pub struct TradeEffectorArgs {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String,
}

/// Errors that can occur during trade
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
            self.client.value().rpc_client(),
            self.wallet.value().solana(),
            &args.input_mint,
            &args.output_mint,
            &args.amount,
        )
        .map_err(|_| TradeError::ClientError)
    }
}
