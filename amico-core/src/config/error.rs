#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to load config")]
    FailedToLoadToml(#[from] toml::de::Error),
}
