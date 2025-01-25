#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to load config")]
    FailedToLoad(#[from] serde_json::Error),
}
