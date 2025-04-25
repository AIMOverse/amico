#[derive(thiserror::Error, Debug)]
pub enum NetworkError {
    #[error("NoStr client error: {0}")]
    NostrClientError(#[from] nostr_sdk::client::Error),

    #[error("Crypto error: {0}")]
    CryptoError(#[from] crate::a2a::crypto::CryptoError),

    #[error("NoStr tag error: {0}")]
    TagError(#[from] nostr::event::tag::Error),
}
