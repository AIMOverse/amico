//! Agent Daemon — local agent example
//!
//! This binary starts an agent daemon that:
//! - Uses an OpenAI-compatible chat model (from the `amico-openai` plugin)
//! - Exposes an HTTP/SSE transport so any UI can connect
//! - Stores sessions on the local filesystem (application-level concern)
//! - Defines its own tools using `amico_system::Tool` (tools are user-defined)
//! - Can run workflows in the background even when the UI disconnects
//!
//! ## Architecture alignment
//!
//! Following the Vercel AI SDK pattern:
//! - **Model services** come from plugin crates (`amico-openai`)
//! - **Tools** are written by the agent developer (`tool.rs`)
//! - **Session storage** is an application concern (`session.rs`)
//! - **Agent presets** come from the framework (`amico-workflows`)

mod handler;
mod session;
mod tool;
mod transport;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::EnvFilter;

// Model from plugin crate (not the framework)
use amico_openai::OpenAiChatModel;
// Tool and session defined locally in this agent
use session::FileSessionStore;
use tool::ShellTool;

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

    // --- Assemble the agent ---
    // Model from plugin; tools and session store defined locally
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
mod anyhow_dep {
    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Result<T> = std::result::Result<T, Error>;
}
pub(crate) use anyhow_dep as anyhow;
