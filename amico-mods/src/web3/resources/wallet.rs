use alloy::signers::{
    k256::Secp256k1,
    local::{LocalSigner, LocalSignerError},
};
use bip39::{Language, Mnemonic, MnemonicType, Seed};

// Ethereum

#[cfg(feature = "web3-ethereum")]
use alloy::signers::local::{
    coins_bip39::English as AlloyEnglish, MnemonicBuilder as AlloyMnemonicBuilder,
};

#[cfg(feature = "web3-ethereum")]
use ecdsa::SigningKey;

/// The signer type for Ethereum wallets
#[cfg(feature = "web3-ethereum")]
pub type EthereumSigner = LocalSigner<SigningKey<Secp256k1>>;

// Solana

#[cfg(feature = "web3-solana")]
use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

/// A wallet containing a mnemonic phrase and optional Ethereum and Solana signers
#[derive(Debug)]
pub struct Wallet {
    mnemonic: Mnemonic,

    #[cfg(feature = "web3-solana")]
    solana_keypair: Keypair,

    #[cfg(feature = "web3-ethereum")]
    ethereum_wallet: EthereumSigner,
}

impl Wallet {
    /// Creates a new wallet with a randomly generated mnemonic phrase.
    pub fn new() -> Result<Self, WalletError> {
        // Generate a 12-word mnemonic phrase
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        tracing::info!("Generated mnemonic phrase: {}", mnemonic.phrase());

        // Create a wallet from the mnemonic phrase
        Self::from_mnemonic(mnemonic)
    }

    /// Creates a new wallet from an existing mnemonic phrase.
    pub fn from_phrase(phrase: &str) -> Result<Self, WalletError> {
        Mnemonic::validate(phrase, Language::English)?;

        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)?;

        Self::from_mnemonic(mnemonic)
    }

    /// Creates a new wallet from an existing mnemonic instance.
    ///
    /// This method is also used by [`from_phrase`][Wallet::from_phrase]
    /// and [`new`][Wallet::new]
    pub fn from_mnemonic(mnemonic: Mnemonic) -> Result<Self, WalletError> {
        #[cfg(feature = "web3-solana")]
        let solana_keypair = Self::load_solana_keypair(&mnemonic)?;

        #[cfg(feature = "web3-ethereum")]
        let ethereum_wallet = Self::load_ethereum_wallet(&mnemonic)?;

        Ok(Self {
            mnemonic,

            #[cfg(feature = "web3-solana")]
            solana_keypair,

            #[cfg(feature = "web3-ethereum")]
            ethereum_wallet,
        })
    }

    /// Returns the mnemonic phrase.
    pub fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    /// Returns the seed for the wallet.
    pub fn seed(&self) -> Seed {
        Seed::new(&self.mnemonic, "")
    }

    /// Saves the mnemonic phrase to a file.
    pub fn save(&self, path: &str) -> Result<(), WalletError> {
        std::fs::write(path, self.phrase())?;
        Ok(())
    }

    /// Loads a wallet from a file containing a mnemonic phrase.
    pub fn load(path: &str) -> Result<Self, WalletError> {
        let phrase = std::fs::read_to_string(path)?;
        Self::from_phrase(&phrase)
    }

    /// Loads a wallet from a file containing a mnemonic phrase, or generates a new one if the file does not exist.
    pub fn load_or_save_new(path: &str) -> Result<Self, WalletError> {
        // If file exists, load it
        if std::path::Path::new(path).exists() {
            Self::load(path)
        } else {
            // If file does not exist, generate a new wallet and write it to the file
            let wallet = Self::new()?;
            wallet.save(path)?;
            Ok(wallet)
        }
    }

    /// Returns the Solana keypair for the wallet.
    #[cfg(feature = "web3-solana")]
    pub fn solana_keypair(&self) -> &Keypair {
        &self.solana_keypair
    }

    /// Returns the Ethereum wallet for the wallet.
    #[cfg(feature = "web3-ethereum")]
    pub fn ethereum_wallet(&self) -> &EthereumSigner {
        &self.ethereum_wallet
    }

    /// Loads a Solana keypair from the mnemonic phrase.
    #[cfg(feature = "web3-solana")]
    pub fn load_solana_keypair(mnemonic: &Mnemonic) -> Result<Keypair, WalletError> {
        let seed = Seed::new(mnemonic, "");
        let keypair = Keypair::from_seed(seed.as_bytes())
            .map_err(|err| WalletError::SolanaKeyPairError(err))?;
        Ok(keypair)
    }

    /// Loads an Ethereum wallet from the mnemonic phrase.
    #[cfg(feature = "web3-ethereum")]
    pub fn load_ethereum_wallet(mnemonic: &Mnemonic) -> Result<EthereumSigner, WalletError> {
        let phrase = mnemonic.phrase();
        let signer = AlloyMnemonicBuilder::<AlloyEnglish>::default()
            .phrase(phrase)
            .build()
            .map_err(|err| WalletError::EthereumSignerError(err))?;
        Ok(signer)
    }

    /// Prints the public keys for both Solana and Ethereum.
    pub fn print_all_pubkeys(&self) {
        #[cfg(feature = "web3-solana")]
        {
            let keypair = self.solana_keypair();
            println!("- Solana: {}", keypair.pubkey());
        }

        #[cfg(feature = "web3-ethereum")]
        {
            let wallet = self.ethereum_wallet();
            println!("- Ethereum: {}", wallet.address());
        }
    }
}

/// Error type for wallet operations.
#[derive(Debug, thiserror::Error)]
pub enum WalletError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Mnemonic error: {0}")]
    MnemonicError(#[from] bip39::ErrorKind),

    #[cfg(feature = "web3-ethereum")]
    #[error("Ethereum signer error: {0}")]
    EthereumSignerError(#[from] LocalSignerError),

    #[cfg(feature = "web3-solana")]
    #[error("Solana keypair error: {0}")]
    SolanaKeyPairError(Box<dyn std::error::Error>),
}

#[cfg(test)]
mod tests {
    use super::*;

    use amico::resource::Resource;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new().expect("Failed to create new wallet");

        // Check that the mnemonic phrase is valid
        let phrase = wallet.phrase();
        assert!(Mnemonic::validate(phrase, Language::English).is_ok());

        // Check that the mnemonic has 12 words
        assert_eq!(phrase.split_whitespace().count(), 12);

        // Check that the seed is generated correctly
        let seed = wallet.seed();
        assert!(!seed.as_bytes().is_empty());
    }

    #[test]
    fn test_from_phrase() {
        // Valid mnemonic phrase
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let wallet = Wallet::from_phrase(phrase).expect("Failed to create wallet from phrase");

        // Check that the phrase is stored correctly
        assert_eq!(wallet.phrase(), phrase);

        // Invalid mnemonic phrase
        let invalid_phrase = "invalid mnemonic phrase";
        assert!(Wallet::from_phrase(invalid_phrase).is_err());
    }

    #[test]
    fn test_from_mnemonic() {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let wallet =
            Wallet::from_mnemonic(mnemonic.clone()).expect("Failed to create wallet from mnemonic");

        // Check that the mnemonic is stored correctly
        assert_eq!(wallet.phrase(), mnemonic.phrase());
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("wallet.txt").to_str().unwrap().to_string();

        // Create and save a wallet
        let original_wallet = Wallet::new().expect("Failed to create new wallet");
        original_wallet
            .save(&file_path)
            .expect("Failed to save wallet");

        // Check that the file exists and contains the phrase
        assert!(Path::new(&file_path).exists());
        let saved_phrase = fs::read_to_string(&file_path).expect("Failed to read wallet file");
        assert_eq!(saved_phrase, original_wallet.phrase());

        // Load the wallet and check that it has the same phrase
        let loaded_wallet = Wallet::load(&file_path).expect("Failed to load wallet");
        assert_eq!(loaded_wallet.phrase(), original_wallet.phrase());
    }

    #[test]
    fn test_load_or_save_new() {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir
            .path()
            .join("new_wallet.txt")
            .to_str()
            .unwrap()
            .to_string();

        // File doesn't exist, so a new wallet should be created and saved
        let wallet1 =
            Wallet::load_or_save_new(&file_path).expect("Failed to load or create wallet");
        assert!(Path::new(&file_path).exists());

        // File exists, so the wallet should be loaded
        let wallet2 =
            Wallet::load_or_save_new(&file_path).expect("Failed to load or create wallet");
        assert_eq!(wallet2.phrase(), wallet1.phrase());
    }

    #[test]
    fn test_error_handling() {
        // Test IO error
        let nonexistent_path = "/nonexistent/path/wallet.txt";
        let result = Wallet::load(nonexistent_path);
        assert!(matches!(result, Err(WalletError::IoError(_))));

        // Test mnemonic error
        let invalid_phrase = "invalid mnemonic phrase";
        let result = Wallet::from_phrase(invalid_phrase);
        assert!(matches!(result, Err(WalletError::MnemonicError(_))));
    }

    #[cfg(feature = "web3-solana")]
    #[test]
    fn test_solana_keypair() {
        let wallet = Wallet::new().expect("Failed to create new wallet");

        // Check that the Solana keypair is generated
        let keypair = wallet.solana_keypair();
        assert!(!keypair.pubkey().to_string().is_empty());

        // Test loading a keypair from a mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let keypair =
            Wallet::load_solana_keypair(&mnemonic).expect("Failed to load Solana keypair");
        assert!(!keypair.pubkey().to_string().is_empty());
    }

    #[cfg(feature = "web3-ethereum")]
    #[test]
    fn test_ethereum_wallet() {
        let wallet = Wallet::new().expect("Failed to create new wallet");

        // Check that the Ethereum wallet is generated
        let eth_wallet = wallet.ethereum_wallet();
        assert!(!eth_wallet.address().to_string().is_empty());

        // Test loading an Ethereum wallet from a mnemonic
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let eth_wallet =
            Wallet::load_ethereum_wallet(&mnemonic).expect("Failed to load Ethereum wallet");
        assert!(!eth_wallet.address().to_string().is_empty());
    }

    #[test]
    fn test_print_all_pubkeys() {
        // This is a simple test to ensure the method doesn't panic
        let wallet = Wallet::new().expect("Failed to create new wallet");
        wallet.print_all_pubkeys();
        // No assertion needed, just checking that it doesn't panic
    }

    #[test]
    fn test_wallet_resource() {
        // Create a new wallet
        let wallet = Wallet::new().expect("Failed to create new wallet");
        let phrase = wallet.phrase().to_string();

        // Create a resource with the wallet
        let wallet_resource = Resource::new("my_wallet".to_string(), wallet);

        // Test resource name
        assert_eq!(wallet_resource.name(), "my_wallet");

        // Test accessing wallet methods through the resource
        let resource_phrase = wallet_resource.borrow_then(|w| w.phrase().to_string());
        assert_eq!(resource_phrase, phrase);

        // Test that we can access wallet functionality through the resource
        #[cfg(feature = "web3-solana")]
        {
            let solana_pubkey =
                wallet_resource.borrow_then(|w| w.solana_keypair().pubkey().to_string());
            assert!(!solana_pubkey.is_empty());
        }

        #[cfg(feature = "web3-ethereum")]
        {
            let eth_address =
                wallet_resource.borrow_then(|w| w.ethereum_wallet().address().to_string());
            assert!(!eth_address.is_empty());
        }
    }
}
