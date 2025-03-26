//! Temporary sensor / effector tools.
//! Wait for `Environment` model to support them.

use amico::ai::errors::ToolCallError;
use amico::ai::tool::{Tool, ToolBuilder};
use amico::environment::{Effector, Sensor};
use amico::resource::Resource;
use amico_mods::web3::solana::balance::{BalanceSensor, BalanceSensorArgs};
use amico_mods::web3::solana::trade::{TradeEffector, TradeEffectorArgs};
use serde_json::json;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;

pub fn balance_sensor_tool(sensor: Resource<BalanceSensor>, pubkey: &Pubkey) -> Tool {
    // `Pubkey` implements the `Copy` trait, so we can just copy it
    let pubkey = *pubkey;
    let sensor = sensor.clone();

    ToolBuilder::new()
        .name("balance_sensor")
        .description("Get the balance of your own Solana account.")
        .parameters(serde_json::json!({}))
        .build_async(move |args| {
            // Clone the sensor and pubkey to move into the async block
            let sensor = sensor.clone();
            let pubkey = pubkey;
            let args = args.clone();

            // Return a boxed future that is both Send and Sync
            async move {
                sensor
                    .value()
                    .sense(BalanceSensorArgs { pubkey })
                    .await
                    .map_err(|err| ToolCallError::ExecutionError {
                        tool_name: "balance_sensor".to_string(),
                        params: args,
                        reason: err.to_string(),
                    })
                    .map(|result| {
                        serde_json::json!({
                            "balance": result.lamports as f64 / LAMPORTS_PER_SOL as f64
                        })
                    })
            }
        })
}

pub fn trade_effector_tool(effector: Resource<TradeEffector>) -> Tool {
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
                    .value()
                    .effect(effector_args)
                    .await
                    .map(|_| json!({"status": "success"}))
                    .map_err(|err| ToolCallError::ExecutionError {
                        tool_name: "trade_solana_token".to_string(),
                        params: args,
                        reason: err.to_string(),
                    })
            }
        })
}
