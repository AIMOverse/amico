use anyhow::Ok;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

pub fn raydium_buy(buyer: &Keypair, mint: &Pubkey, amount: u64) -> anyhow::Result<()> {
    let swap_tx_url = "https://share.raydium.io/dialect/actions/swap/tx";
    let swap_tx_params = format!(
        "inputMint=sol&outputMint={}&amount={}",
        mint.to_string(),
        amount
    );

    // Construct the full URL
    let full_url = format!("{}?{}", swap_tx_url, swap_tx_params);

    // TODO: Send the request to the Raydium API
    tracing::info!("{} Calling raydium_buy {:?}", buyer.pubkey(), full_url);

    Ok(())
}
