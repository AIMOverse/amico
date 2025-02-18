use anyhow::Ok;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signature::Keypair, signer::Signer,
    transaction::VersionedTransaction,
};

use crate::utils::solana::get_rpc_url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SwapTxResult {
    transaction: String,
}

pub fn raydium_buy(buyer: &Keypair, mint: &Pubkey, amount: u64) -> anyhow::Result<String> {
    let swap_tx_url = "https://share.raydium.io/dialect/actions/swap/tx";
    let swap_tx_params = format!(
        "inputMint=sol&outputMint={}&amount={}",
        mint,
        amount as f64 / LAMPORTS_PER_SOL as f64
    );

    // Construct the full URL
    let full_url = format!("{}?{}", swap_tx_url, swap_tx_params);
    tracing::debug!("Full URL: {}", full_url);

    // Send the request to the Raydium API
    let res = ureq::post(full_url)
        .send_json(json!({
            "account": buyer.pubkey().to_string(),
        }))?
        .body_mut()
        .read_json::<SwapTxResult>()?;

    tracing::debug!("Swap transaction: {:?}", res);

    // Decode the transaction
    let decoded_bytes = STANDARD.decode(res.transaction.as_str())?;
    let versioned_tx = bincode::deserialize::<VersionedTransaction>(&decoded_bytes)?;

    tracing::debug!("Decoded transaction: {:#?}", versioned_tx);

    let tx = VersionedTransaction::try_new(versioned_tx.message.clone(), &[buyer])?;

    // Connect to the Solana cluster (e.g., mainnet-beta)
    let rpc_url = get_rpc_url();
    let client = RpcClient::new(rpc_url);

    let sig = client.send_and_confirm_transaction(&tx)?;

    tracing::info!("Transaction signature: {}", sig);

    Ok(sig.to_string())
}
