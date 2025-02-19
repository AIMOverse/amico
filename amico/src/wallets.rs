// use std::str::FromStr;

use alloy::signers::local::{
    coins_bip39::English as AlloyEnglish, MnemonicBuilder as AlloyMnemonicBuilder, PrivateKeySigner,
};
// use aptos_sdk::{
//     rest_client::FaucetClient as AptosFaucetClient, types::LocalAccount as AptosLocalAccount,
// };
use bip39::{Language, Mnemonic, MnemonicType, Seed};
// use once_cell::sync::Lazy;
use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};
// use url::Url;

pub struct AgentWallet {
    mnemonic: Mnemonic,
}

impl AgentWallet {
    pub fn new() -> Self {
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        tracing::info!("Generated mnemonic: {}", mnemonic.phrase());
        Self { mnemonic }
    }

    pub fn phrase(&self) -> &str {
        self.mnemonic.phrase()
    }

    pub fn from_phrase(phrase: &str) -> anyhow::Result<Self> {
        if let Err(e) = Mnemonic::validate(phrase, Language::English) {
            return Err(anyhow::anyhow!("Invalid mnemonic phrase: {}", e));
        }

        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)?;

        Ok(Self { mnemonic })
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        std::fs::write(path, self.phrase())?;
        Ok(())
    }

    pub fn load(path: &str) -> anyhow::Result<Self> {
        let phrase = std::fs::read_to_string(path)?;
        Self::from_phrase(&phrase)
    }

    pub fn load_or_save_new(path: &str) -> anyhow::Result<Self> {
        // If file exists, load it
        if std::path::Path::new(path).exists() {
            Self::load(path)
        } else {
            // If file does not exist, generate a new wallet and write it to the file
            let wallet = Self::new();
            wallet.save(path)?;
            Ok(wallet)
        }
    }

    pub fn seed(&self) -> Seed {
        Seed::new(&self.mnemonic, "")
    }

    pub fn solana_keypair(&self) -> anyhow::Result<Keypair> {
        let seed = self.seed();
        Keypair::from_seed(seed.as_bytes())
            .map_err(|err| anyhow::anyhow!("Error generating keypair: {}", err))
    }

    pub fn ethereum_wallet(&self) -> anyhow::Result<PrivateKeySigner> {
        let phrase = self.phrase();
        let wallet = AlloyMnemonicBuilder::<AlloyEnglish>::default()
            .phrase(phrase)
            .build()
            .map_err(|err| anyhow::anyhow!("Error generating wallet: {}", err))?;
        Ok(wallet)
    }

    pub async fn print_all_pubkeys(&self) -> anyhow::Result<()> {
        let keypair = self.solana_keypair()?;
        println!("- Solana: {}", keypair.pubkey());

        let wallet = self.ethereum_wallet()?;
        println!("- Ethereum: {}", wallet.address());

        // let aptos_account = self.aptos_account().await?;
        // println!("- Aptos: {}", aptos_account.address());

        Ok(())
    }

    // pub async fn aptos_account(&self) -> anyhow::Result<AptosLocalAccount> {
    //     let account = AptosLocalAccount::from_derive_path("m/44'/637'/0'/0'/0'", self.phrase(), 0)?;

    //     if let Err(e) = self.create_aptos_account(&account).await {
    //         tracing::error!("Error creating account: {}", e);
    //     }

    //     Ok(account)
    // }

    // async fn create_aptos_account(&self, account: &AptosLocalAccount) -> anyhow::Result<()> {
    //     let faucet_client =
    //         AptosFaucetClient::new(APTOS_FAUCET_URL.clone(), APTOS_NODE_URL.clone()); // <:!:section_1a

    //     faucet_client.fund(account.address(), 100_000).await?;

    //     tracing::debug!("Created account: {}", account.address());

    //     Ok(())
    // }
}

// static APTOS_NODE_URL: Lazy<Url> = Lazy::new(|| {
//     Url::from_str(
//         std::env::var("APTOS_NODE_URL")
//             .as_ref()
//             .map(|s| s.as_str())
//             .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
//     )
//     .unwrap()
// });

// static APTOS_FAUCET_URL: Lazy<Url> = Lazy::new(|| {
//     Url::from_str(
//         std::env::var("APTOS_FAUCET_URL")
//             .as_ref()
//             .map(|s| s.as_str())
//             .unwrap_or("https://faucet.devnet.aptoslabs.com"),
//     )
//     .unwrap()
// });
