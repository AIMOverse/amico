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
        *wallet.wallet.value().solana(),
        *loaded_wallet.wallet.value().solana()
    );
}

#[test]
fn test_create_agent() {
    // Create the provider
    let provider = WasmProvider::new("https://api.openai.com", "sk-123");

    // Create the wallet
    let _wallet = WasmWallet::new().unwrap();

    // Create the service
    let _service = WasmStdService::new(provider, "gpt-4o", "Amico", 0.2, 1000, vec![]);
}
