use bip39::{Language, Mnemonic, MnemonicType, Seed};
use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

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

    pub fn print_all_pubkeys(&self) -> anyhow::Result<()> {
        let keypair = self.solana_keypair()?;
        println!("- Solana: {}", keypair.pubkey());
        Ok(())
    }
}
