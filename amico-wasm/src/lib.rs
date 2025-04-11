//! WASM bindings for Amico

#[cfg(test)]
mod tests;

use amico::{
    ai::{
        services::{CompletionService, ServiceBuilder},
        tool::Tool,
    },
    resource::Resource,
};
use amico_mods::{
    std::ai::{
        providers::rig::{RigProvider, providers},
        services::InMemoryService,
    },
    web3::{
        solana::{
            balance::BalanceSensor,
            resources::{SolanaClient, SolanaClientResource},
            trade::TradeEffector,
        },
        wallet::Wallet,
    },
};
use wasm_bindgen::prelude::*;

/// WASM wrapper for `Tool`.
#[wasm_bindgen]
pub struct WasmTool {
    pub(crate) tool: Tool,
}

/// WASM wrapper for `InMemoryService<RigProvider>`.
#[wasm_bindgen]
pub struct WasmStdService {
    pub(crate) service: InMemoryService<RigProvider>,
}

#[wasm_bindgen]
impl WasmStdService {
    /// Creates a new `WasmStdService`.
    ///
    /// # Arguments
    ///
    /// * `provider` - The provider to use for the service.
    /// * `model_name` - The name of the model to use.
    /// * `system_prompt` - The system prompt to use.
    /// * `temperature` - The temperature to use.
    /// * `max_tokens` - The maximum number of tokens to use.
    /// * `tools` - The tools to use.
    ///
    /// # Returns
    ///
    /// A new `WasmStdService`.
    #[wasm_bindgen(constructor)]
    pub fn new(
        provider: WasmProvider,
        model_name: &str,
        system_prompt: &str,
        temperature: f64,
        max_tokens: u64,
        tools: Vec<WasmTool>,
    ) -> Self {
        let service_builder = ServiceBuilder::new(provider.provider)
            .model(model_name.to_string())
            .system_prompt(system_prompt.to_string())
            .temperature(temperature)
            .max_tokens(max_tokens)
            .tools(tools.iter().map(|t| t.tool.clone()).collect());
        let service = service_builder.build::<InMemoryService<RigProvider>>();
        Self { service }
    }

    /// Generates text based on a prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The prompt to generate text from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the generated text or an error.
    #[wasm_bindgen]
    pub async fn chat(&mut self, prompt: &str) -> Result<String, String> {
        self.service
            .generate_text(prompt.to_string())
            .await
            .map_err(|e| e.to_string())
    }
}

/// WASM wrapper for `RigProvider`.
#[wasm_bindgen]
pub struct WasmProvider {
    pub(crate) provider: RigProvider,
}

#[wasm_bindgen]
impl WasmProvider {
    /// Creates a new `WasmProvider`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the provider.
    /// * `api_key` - The API key for the provider.
    ///
    /// # Returns
    ///
    /// A new `WasmProvider`.
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            provider: RigProvider::openai(providers::openai::Client::from_url(api_key, base_url)),
        }
    }
}

/// WASM wrapper for Solana Client.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WasmSolanaClient {
    pub(crate) client: SolanaClientResource,
}

#[wasm_bindgen]
impl WasmSolanaClient {
    /// Creates a new `WasmClient`.
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The RPC URL for the client.
    ///
    /// # Returns
    ///
    /// A new `WasmClient`.
    #[wasm_bindgen(constructor)]
    pub fn new(rpc_url: &str) -> Self {
        Self {
            client: SolanaClientResource::new(
                "Solana RPC client".to_string(),
                SolanaClient::new(rpc_url),
            ),
        }
    }
}

/// WASM wrapper for `BalanceSensor`.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WasmBalanceSensor {
    pub(crate) sensor: Resource<BalanceSensor>,
}

#[wasm_bindgen]
impl WasmBalanceSensor {
    /// Creates a new `WasmBalanceSensor`.
    ///
    /// # Arguments
    ///
    /// * `client` - The Solana client resource.
    /// * `wallet` - The wallet resource.
    ///
    /// # Returns
    ///
    /// A new `WasmBalanceSensor`.
    #[wasm_bindgen(constructor)]
    pub fn new(client: WasmSolanaClient, wallet: WasmWallet) -> Self {
        Self {
            sensor: Resource::new(
                "balance_sensor".to_string(),
                BalanceSensor::new(client.client, wallet.wallet),
            ),
        }
    }

    /// Returns a tool for getting the balance of the agent's wallet.
    ///
    /// # Returns
    ///
    /// A `WasmTool` containing the balance sensor's agent wallet balance tool.
    #[wasm_bindgen]
    pub fn agent_wallet_balance_tool(&self) -> WasmTool {
        WasmTool {
            tool: self.sensor.value().agent_wallet_balance_tool(),
        }
    }

    /// Returns a tool for getting the balance of a specific account.
    ///
    /// # Returns
    ///
    /// A `WasmTool` containing the balance sensor's account balance tool.
    #[wasm_bindgen]
    pub fn account_balance_tool(&self) -> WasmTool {
        WasmTool {
            tool: self.sensor.value().account_balance_tool(),
        }
    }
}

/// WASM wrapper for `TradeEffector`.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WasmTradeEffector {
    pub(crate) effector: Resource<TradeEffector>,
}

#[wasm_bindgen]
impl WasmTradeEffector {
    /// Creates a new `WasmTradeEffector`.
    ///
    /// # Arguments
    ///
    /// * `client` - The Solana client resource.
    /// * `wallet` - The wallet resource.
    ///
    /// # Returns
    ///
    /// A new `WasmTradeEffector`.
    #[wasm_bindgen(constructor)]
    pub fn new(client: WasmSolanaClient, wallet: WasmWallet) -> Self {
        Self {
            effector: Resource::new(
                "trade_effector".to_string(),
                TradeEffector::new(client.client, wallet.wallet),
            ),
        }
    }

    /// Returns a tool for executing trades.
    ///
    /// # Returns
    ///
    /// A `WasmTool` containing the trade effector's tool.
    #[wasm_bindgen]
    pub fn tool(&self) -> WasmTool {
        WasmTool {
            tool: self.effector.value().tool(),
        }
    }
}

/// WASM wrapper for `Wallet`.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WasmWallet {
    pub(crate) wallet: Resource<Wallet>,
}

#[wasm_bindgen]
impl WasmWallet {
    /// Creates a new `WasmWallet`.
    ///
    /// # Returns
    ///
    /// A new `WasmWallet`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Self, String> {
        let wallet = Wallet::new().map_err(|e| e.to_string())?;
        Ok(Self {
            wallet: Resource::new("wallet".to_string(), wallet),
        })
    }

    /// Loads a wallet from a mnemonic phrase.
    ///
    /// # Arguments
    ///
    /// * `phrase` - The mnemonic phrase to load the wallet from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded `WasmWallet` or an error.
    pub fn load(phrase: &str) -> Result<Self, String> {
        let wallet = Wallet::from_phrase(phrase).map_err(|e| e.to_string())?;
        Ok(Self {
            wallet: Resource::new("wallet".to_string(), wallet),
        })
    }

    /// Returns the mnemonic phrase of the wallet.
    ///
    /// # Returns
    ///
    /// The mnemonic phrase.
    #[wasm_bindgen]
    pub fn phrase(&self) -> String {
        self.wallet.value().phrase().to_string()
    }
}
