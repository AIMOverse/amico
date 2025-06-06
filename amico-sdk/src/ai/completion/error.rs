#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Model error: {0}")]
    Model(String),

    #[error("Model name {0} is not available")]
    ModelNameNotAvailable(String),

    #[error("Bad response: {0}")]
    BadResponse(String),

    #[error("Tool error: {0}")]
    Tool(String),
}
