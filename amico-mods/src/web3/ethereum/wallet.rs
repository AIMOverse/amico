use alloy::signers::local::LocalSignerError;
use alloy::signers::local::{MnemonicBuilder, coins_bip39::English};
use alloy::signers::{k256::Secp256k1, local::LocalSigner};
use bip39::Mnemonic;
use ecdsa::SigningKey;

use crate::web3::wallet::{WalletComponent, WalletError};

/// The local signer type alias
pub type EthereumSigner = LocalSigner<SigningKey<Secp256k1>>;

/// A wrapper around the local signer that implements the `WalletComponent` trait.
pub struct EthereumWallet(EthereumSigner);

/// Error type for Ethereum wallet operations.
#[derive(Debug, thiserror::Error)]
pub enum EthereumWalletError {
    #[error("Ethereum local signer error: {0}")]
    LocalSignerError(#[from] LocalSignerError),
}

impl WalletComponent for EthereumWallet {
    type Signer = EthereumSigner;

    fn from_mnemonic(mnemonic: &Mnemonic) -> Result<Self, WalletError> {
        let phrase = mnemonic.phrase();
        let signer = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .build()
            .map_err(EthereumWalletError::LocalSignerError)?;
        Ok(EthereumWallet(signer))
    }

    fn pubkey(&self) -> String {
        self.0.address().to_string()
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
        let eth_wallet =
            EthereumWallet::from_mnemonic(&mnemonic).expect("Failed to load Ethereum wallet");
        assert!(!eth_wallet.pubkey().to_string().is_empty());
    }
}
