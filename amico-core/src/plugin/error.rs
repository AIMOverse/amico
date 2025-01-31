#[derive(thiserror::Error, Debug)]
pub enum PluginError {
    #[error("Failed to load plugin")]
    FailedToLoad(#[from] std::io::Error),
}
