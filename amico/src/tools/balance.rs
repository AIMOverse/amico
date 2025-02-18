use alloy::signers::local::PrivateKeySigner;
use amico::ai::{errors::ToolCallError, tool::Tool};
use serde_json::json;
use solana_sdk::{native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer};

pub fn check_solana_balance(keypair: Keypair) -> Tool {
    Tool {
        name: "check_solana_balance".to_string(),
        description: "Check SOL balance on Solana in your own wallet".to_string(),
        parameters: json!({}),
        tool_call: Box::new(move |_| {
            tracing::info!("Calling check_solana_balance tool");
            tracing::debug!("Keypair: {}", keypair.pubkey());

            // Check balance
            let balance = crate::utils::solana::get_balance_lamports(&keypair.pubkey().to_string())
                .map_err(|err| {
                    tracing::error!("Error checking balance: {}", err);
                    ToolCallError::ExecutionError {
                        tool_name: "check_solana_balance".to_string(),
                        params: json!({}),
                        reason: err.to_string(),
                    }
                })?;

            // Convert balance to SOL
            let balance_sol = balance as f64 / LAMPORTS_PER_SOL as f64;

            tracing::debug!("Balance: {} SOL", balance_sol);

            Ok(json!({
                "balance": format!("{} SOL", balance_sol),
            }))
        }),
    }
}

pub fn check_ethereum_balance(wallet: PrivateKeySigner) -> Tool {
    Tool {
        name: "check_ethereum_balance".to_string(),
        description: "Check ETH balance on Ethereum in your own wallet".to_string(),
        parameters: json!({}),
        tool_call: Box::new(move |_| {
            tracing::info!("Calling check_ethereum_balance tool");
            Ok(json!({
                "balance": format!("{} ETH", crate::utils::eth::check_eth_balance(&wallet.address()).unwrap()),
            }))
        }),
    }
}
