use alloy::signers::local::PrivateKeySigner;
use amico::ai::{errors::ToolCallError, tool::Tool};
use mpl_core::instructions::CreateV1Builder;
use serde_json::json;
use solana_client::rpc_client;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::utils::solana::get_rpc_url;

pub fn search_jokes_tool() -> Tool {
    Tool {
        name: "search_for_jokes".to_string(),
        description: "Search for jokes".to_string(),
        parameters: json!({}),
        tool_call: Box::new(|_| {
            tracing::info!("Calling search_for_jokes tool");
            Ok(json!({
                "jokes": [
                    "Why don't scientists trust atoms?\nBecause they make up everything!",
                    "Why do programmers prefer dark mode?\nBecause the light attracts bugs!",
                    "Why did the TCP connection break up with UDP?\nBecause TCP wanted a reliable connection, but UDP just couldn't commit!",
                    "Why do UDP packets never get invited to parties?\nBecause they never respond to invites!",
                ],
            }))
        }),
    }
}

pub fn check_solana_balance(keypair: Keypair) -> Tool {
    Tool {
        name: "check_solana_balance".to_string(),
        description: "Check SOL balance on Solana in your own wallet".to_string(),
        parameters: json!({}),
        tool_call: Box::new(move |_| {
            tracing::info!("Calling check_solana_balance tool");
            tracing::debug!("Keypair: {}", keypair.pubkey());

            // Check balance
            let balance = crate::utils::solana::get_balance_in_sol(&keypair.pubkey().to_string())
                .map_err(|err| {
                tracing::error!("Error checking balance: {}", err);
                ToolCallError::ExecutionError {
                    tool_name: "check_solana_balance".to_string(),
                    params: json!({}),
                    reason: err.to_string(),
                }
            })?;
            tracing::debug!("Balance: {} SOL", balance);

            Ok(json!({
                "balance": format!("{} SOL", balance),
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

pub fn create_asset_tool(keypair: Keypair) -> Tool {
    Tool {
        name: "create_asset".to_string(),
        description: "Create a NFT on Solana representing yourself".to_string(),
        parameters: json!({}),
        tool_call: Box::new(move |_| {
            tracing::info!("Calling create_asset tool");

            // Create the NFT
            let rpc_client = rpc_client::RpcClient::new(get_rpc_url());

            let asset = Keypair::new();

            let create_asset_ix = CreateV1Builder::new()
                .asset(asset.pubkey())
                .payer(keypair.pubkey())
                .name("Agent NFT".into())
                .uri("https://cyan-acute-python-533.mypinata.cloud/ipfs/QmR8n52jtQMZJgYkBFWszhzCXkC9L6qpdSz6omWwtRwLgs".into())
                .instruction();

            let signers = vec![&asset, &keypair];

            let last_blockhash =
                rpc_client
                    .get_latest_blockhash()
                    .map_err(|err| ToolCallError::ExecutionError {
                        tool_name: "create_asset".to_string(),
                        params: json!({}),
                        reason: err.to_string(),
                    })?;

            let create_asset_tx = Transaction::new_signed_with_payer(
                &[create_asset_ix],
                Some(&keypair.pubkey()),
                &signers,
                last_blockhash,
            );

            match rpc_client.send_and_confirm_transaction(&create_asset_tx) {
                Ok(res) => {
                    tracing::info!("NFT created. Signature: {:?}", res);

                    Ok(json!({
                        "message": "NFT created successfully",
                        "signature": res.to_string(),
                    }))
                }
                Err(err) => Err(amico::ai::errors::ToolCallError::ExecutionError {
                    tool_name: "create_asset".to_string(),
                    params: json!({}),
                    reason: format!("Failed to create NFT: {:?}", err),
                }),
            }
        }),
    }
}
