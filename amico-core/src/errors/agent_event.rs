use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentEventError {
    #[error("`serde_json` error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("AgentEvent content error: {0}")]
    ContentError(&'static str),
}
