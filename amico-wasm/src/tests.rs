use super::*;

#[test]
fn test_create_load_wallet() {
    // Create the wallet
    let wallet = WasmWallet::new().unwrap();
    let phrase = wallet.phrase();

    assert!(phrase.len() > 0);

    // Load the wallet from mnemonic phrase
    let loaded_wallet = WasmWallet::load(&phrase).unwrap();

    assert_eq!(wallet.phrase(), loaded_wallet.phrase());
    assert_eq!(
        *wallet.wallet.value().solana_keypair(),
        *loaded_wallet.wallet.value().solana_keypair()
    );
}

#[test]
fn test_create_agent() {
    // Create the provider
    let provider = WasmProvider::new("https://api.openai.com", "sk-123");

    // Create the wallet
    let wallet = WasmWallet::new().unwrap();

    // Create the Solana client
    let solana_client = WasmSolanaClient::new("https://api.devnet.solana.com");

    // Create the balance sensor
    let balance_sensor = WasmBalanceSensor::new(solana_client.clone(), wallet.clone());

    // Create the trade effector
    let trade_effector = WasmTradeEffector::new(solana_client.clone(), wallet.clone());

    // Create the service
    let _service = WasmStdService::new(
        provider,
        "gpt-4o",
        "Amico",
        0.2,
        1000,
        vec![
            balance_sensor.agent_wallet_balance_tool(),
            balance_sensor.account_balance_tool(),
            trade_effector.tool(),
        ],
    );
}
