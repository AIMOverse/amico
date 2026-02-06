//! Agent Daemon — local agent example
//!
//! This binary starts an agent daemon that:
//! - Uses an OpenAI-compatible chat model for reasoning (from `amico_models::openai`)
//! - Exposes an HTTP/SSE transport so any UI can connect
//! - Stores sessions on the local filesystem (from `amico_runtime::fs_store`)
//! - Can run workflows in the background even when the UI disconnects
//! - Supports parallel sessions and stream resume
//!
//! The agent is assembled from framework-provided modules in just a few
//! lines of code — no custom model, tool, or session implementations needed.

mod handler;
mod transport;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::EnvFilter;

// Import concrete implementations directly from the framework
use amico_models::openai::OpenAiChatModel;
use amico_runtime::fs_store::FileSessionStore;
use amico_system::shell::ShellTool;

use handler::AgentChatHandler;

/// Shared application state accessible by the HTTP handlers.
pub struct AppState {
    pub chat_handler: AgentChatHandler,
    pub session_store: RwLock<FileSessionStore>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing (respects RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    // --- Configuration (from env or defaults) ---
    let api_base = std::env::var("OPENAI_API_BASE")
        .unwrap_or_else(|_| "https://api.openai.com/v1".into());
    let api_key = std::env::var("OPENAI_API_KEY").unwrap_or_default();
    let model_name = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".into());
    let data_dir =
        std::env::var("AGENT_DATA_DIR").unwrap_or_else(|_| ".amico/sessions".into());
    let bind_addr = std::env::var("AGENT_BIND").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let system_prompt = std::env::var("AGENT_SYSTEM_PROMPT").unwrap_or_else(|_| {
        "You are a helpful assistant. You can execute shell commands when needed.".into()
    });

    // --- Assemble the agent from framework modules ---
    let model = OpenAiChatModel::new(&api_base, &api_key, &model_name);
    let shell_tool = ShellTool;
    let session_store = FileSessionStore::new(&data_dir).await?;

    let chat_handler = AgentChatHandler::new(model, shell_tool, system_prompt);

    let state = Arc::new(AppState {
        chat_handler,
        session_store: RwLock::new(session_store),
    });

    // --- Start HTTP transport ---
    tracing::info!("Agent daemon listening on {bind_addr}");
    transport::serve(state, &bind_addr).await?;

    Ok(())
}

// We use `anyhow` only inside the binary; framework crates stay error-type generic.
// Adding it here keeps the example concise.
mod anyhow_dep {
    // Re-use `Box<dyn Error>` as a lightweight anyhow-like type so we
    // don't need to add the `anyhow` crate as a dependency.
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Result<T> = std::result::Result<T, Error>;
}
pub(crate) use anyhow_dep as anyhow;
