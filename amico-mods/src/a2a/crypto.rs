use base64::{engine::general_purpose::STANDARD, Engine};
use sodiumoxide::crypto::{
    sealedbox,
    sign::{
        ed25519::{to_curve25519_pk, PublicKey as EdPublicKey, SecretKey as EdSecretKey},
        to_curve25519_sk,
    },
};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Failed to init sodiumoxide")]
    SodiumoxideInitError,

    #[error("Base64 decode error: {0}")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("sodiumoxide sealedbox open error")]
    SealedBoxOpenError,

    #[error("UTF-8 decode error")]
    UTF8DecodeError(#[from] std::string::FromUtf8Error),

    #[error("Invalid public key")]
    InvalidPublicKey,

    #[error("Invalid private key")]
    InvalidPrivateKey,

    #[error("Failed to convert ed25519 key to curve25519")]
    ConvertToCurve25519Error,
}

/// Encrypt a message using a Solana public key
pub fn encrypt_message(plaintext: &str, pubkey: &Pubkey) -> Result<String, CryptoError> {
    // Initialize sodiumoxide for encryption
    sodiumoxide::init().map_err(|_| CryptoError::SodiumoxideInitError)?;
    // Convert receiver Solana pubkey to Curve25519 public key
    let receiver_pubkey_bytes = pubkey.to_bytes();
    let ed_pk =
        EdPublicKey::from_slice(&receiver_pubkey_bytes).ok_or(CryptoError::InvalidPublicKey)?;
    let receiver_curve_pk =
        to_curve25519_pk(&ed_pk).map_err(|_| CryptoError::ConvertToCurve25519Error)?;
    // Encrypt message content
    let ciphertext = sealedbox::seal(plaintext.as_bytes(), &receiver_curve_pk);
    let encrypted_content = STANDARD.encode(&ciphertext);
    Ok(encrypted_content)
}

/// Decrypt a message using a Solana keypair
pub fn decrypt_message(ciphertext: &str, keypair: &Keypair) -> Result<String, CryptoError> {
    // Initialize sodiumoxide and derive Curve25519 keys for decryption
    sodiumoxide::init().map_err(|_| CryptoError::SodiumoxideInitError)?;

    // Conv
    let sk_bytes = keypair.to_bytes();
    let ed_sk = EdSecretKey::from_slice(&sk_bytes).ok_or(CryptoError::InvalidPrivateKey)?;
    let ed_pk = ed_sk.public_key();
    let my_curve_sk =
        to_curve25519_sk(&ed_sk).map_err(|_| CryptoError::ConvertToCurve25519Error)?;
    let my_curve_pk =
        to_curve25519_pk(&ed_pk).map_err(|_| CryptoError::ConvertToCurve25519Error)?;

    // Decode and decrypt the ciphertext
    let decoded = STANDARD
        .decode(&ciphertext)
        .map_err(CryptoError::Base64DecodeError)?;
    let plaintext_bytes = sealedbox::open(&decoded, &my_curve_pk, &my_curve_sk)
        .map_err(|_| CryptoError::SealedBoxOpenError)?;
    let plaintext = String::from_utf8(plaintext_bytes).map_err(CryptoError::UTF8DecodeError)?;
    Ok(plaintext)
}
