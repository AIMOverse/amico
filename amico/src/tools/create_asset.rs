use amico::ai::{errors::ToolCallError, tool::{Tool, ToolDefinition}};
use mpl_core::instructions::CreateV1Builder;
use serde_json::json;
use solana_client::rpc_client;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::utils::solana::get_rpc_url;

pub fn create_asset_tool(keypair: Keypair) -> Tool {
    Tool {
        definition: ToolDefinition {
            name: "create_asset".to_string(),
        description: "Create a NFT on Solana representing yourself".to_string(),
        parameters: json!({}),
        },
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
