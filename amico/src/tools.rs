//! Temporary sensor / effector tools.
//! Wait for `Environment` model to support them.

use amico::ai::errors::ToolCallError;
use amico::ai::tool::{Tool, ToolBuilder};
use amico::environment::Sensor;
use amico::resource::Resource;
use amico_mods::web3::solana::balance::{BalanceSensor, BalanceSensorArgs};
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
