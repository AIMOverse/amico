use std::str::FromStr;

use amico::ai::{
    errors::ToolCallError,
    tool::{Tool, ToolCall},
};
use serde_json::json;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
};

use crate::utils::sol_swap::raydium_buy;

pub fn buy_solana_token_tool(keypair: Keypair) -> Tool {
    Tool {
        name: "buy_solana_token".to_string(),
        description: "Buy a Solana token".to_string(),
        parameters: json!({
            "type": "object",
            "properties": {
                "token_address": {
                    "type": "string",
                    "description": "The address of the token to buy",
                },
                "amount_sol": {
                    "type": "number",
                    "description": "The amount of SOL to spend on the transaction",
                },
            },
            "required": ["token_address", "amount_sol"],
        }),
        tool_call: ToolCall::Sync(Box::new(move |params| {
            tracing::info!("Calling buy_solana_token tool {:?}", params);

            // Parse the parameters
            let token_address =
                params["token_address"]
                    .as_str()
                    .ok_or(ToolCallError::InvalidParam {
                        name: "token_address".to_string(),
                        value: params["token_address"].clone(),
                        reason: "token_address must be a string".to_string(),
                    })?;
            let amount_sol = params["amount_sol"]
                .as_f64()
                .ok_or(ToolCallError::InvalidParam {
                    name: "amount_sol".to_string(),
                    value: params["amount_sol"].clone(),
                    reason: "amount_sol must be a number".to_string(),
                })?;

            // Parse the token address
            let token_mint =
                Pubkey::from_str(token_address).map_err(|err| ToolCallError::InvalidParam {
                    name: "token_address".to_string(),
                    value: params["token_address"].clone(),
                    reason: format!("`token_address` is not a valid Pubkey: {}", err),
                })?;

            // Calculate the amount of lamports to send
            let amount = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;

            // Check balance
            let balance = crate::utils::solana::get_balance_lamports(&keypair.pubkey().to_string())
                .map_err(|err| {
                    tracing::error!("Error checking balance: {}", err);
                    ToolCallError::ExecutionError {
                        tool_name: "buy_solana_token".to_string(),
                        params: json!({}),
                        reason: err.to_string(),
                    }
                })?;

            if balance < amount {
                return Err(ToolCallError::ExecutionError {
                    tool_name: "buy_solana_token".to_string(),
                    params: params.clone(),
                    reason: format!(
                        "Not enough balance: balance({}) < amount({})",
                        balance, amount
                    ),
                });
            }

            if let Err(err) = raydium_buy(&keypair, &token_mint, amount) {
                return Err(ToolCallError::ExecutionError {
                    tool_name: "buy_solana_token".to_string(),
                    params: params.clone(),
                    reason: format!("Error buying token: {}", err),
                });
            }

            Ok(json!({
                "result": "Successfully swapped SOL for token",
            }))
        })),
    }
}
