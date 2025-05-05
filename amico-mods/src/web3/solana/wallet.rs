use bip39::{Mnemonic, Seed};
use solana_sdk::{
    signature::Keypair,
    signer::{SeedDerivable, Signer},
};

use crate::web3::wallet::{WalletComponent, WalletError};

/// A wrapper around the Solana keypair that implements the `WalletComponent` trait.
pub struct SolanaWallet(Keypair);

/// Error type for Solana wallet operations.
#[derive(Debug, thiserror::Error)]
pub enum SolanaWalletError {
    #[error("Solana keypair error: {0}")]
    KeypairError(String),
}

impl WalletComponent for SolanaWallet {
    type Signer = Keypair;

    fn from_mnemonic(mnemonic: &Mnemonic) -> Result<Self, WalletError> {
        let seed = Seed::new(mnemonic, "");
        let keypair = Keypair::from_seed(seed.as_bytes())
            .map_err(|err| SolanaWalletError::KeypairError(err.to_string()))?;
        Ok(SolanaWallet(keypair))
    }

    fn pubkey(&self) -> String {
        self.0.pubkey().to_string()
    }

    fn get(&self) -> &Self::Signer {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bip39::{Language::English, MnemonicType::Words12};

    #[test]
    fn test_from_mnemonic() {
        let mnemonic = Mnemonic::new(Words12, English);
        let keypair =
            SolanaWallet::from_mnemonic(&mnemonic).expect("Failed to load Solana keypair");
        assert!(!keypair.pubkey().to_string().is_empty());
    }
}
