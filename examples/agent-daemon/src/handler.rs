//! Agent chat handler — ties model, tool, and session together.
//!
//! Implements `amico::ChatHandler` by:
//! 1. Loading the session's message history
//! 2. Appending the user message
//! 3. Calling the streaming chat model
//! 4. Buffering the response so it can be resumed if the UI reconnects

use amico::ChatHandler;
use amico_models::openai::{OpenAiChatModel, OpenAiModelError};
use amico_models::{ChatInput, ChatMessage, StreamChunk};
use amico_runtime::fs_store::{FileSession, FileSessionStore, SerializableMessage};
use amico_system::shell::ShellTool;
use futures::stream::BoxStream;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

/// How long to keep a completed workflow run in memory so that
/// reconnecting clients can still read buffered tokens.
const RUN_CLEANUP_DELAY_SECS: u64 = 30;

/// Polling interval for the SSE replay loop. Balances responsiveness
/// (lower = faster delivery of new tokens) against CPU usage.
const SSE_POLL_INTERVAL_MS: u64 = 50;

/// A single streaming workflow run, collecting tokens as they arrive.
pub struct WorkflowRun {
    /// Buffered tokens so far (for resume)
    pub tokens: Vec<StreamChunk>,
    /// Whether the run has completed
    pub done: bool,
}

impl WorkflowRun {
    fn new() -> Self {
        Self {
            tokens: Vec::new(),
            done: false,
        }
    }
}

/// The agent chat handler wires together the model, tools, and session store.
pub struct AgentChatHandler {
    model: OpenAiChatModel,
    #[allow(dead_code)]
    tool: ShellTool,
    system_prompt: String,
    /// Active workflow runs indexed by session id.
    /// Tokens are buffered here so SSE streams can be resumed.
    pub active_runs: Arc<RwLock<HashMap<String, Arc<Mutex<WorkflowRun>>>>>,
}

impl AgentChatHandler {
    pub fn new(model: OpenAiChatModel, tool: ShellTool, system_prompt: String) -> Self {
        Self {
            model,
            tool,
            system_prompt,
            active_runs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a background streaming workflow for the given session.
    ///
    /// Returns immediately — the caller can poll `active_runs` for progress.
    pub async fn start_workflow(
        &self,
        session: &FileSession,
        session_store: &RwLock<FileSessionStore>,
    ) -> Result<(), AgentChatError> {
        use amico_models::StreamingChatModel;

        // Build the prompt from session history
        let mut messages: Vec<ChatMessage> = vec![ChatMessage::system(&self.system_prompt)];
        for m in &session.messages {
            messages.push(ChatMessage::new(
                amico_models::ChatRole::from_str_lossy(&m.role),
                vec![amico_models::ContentPart::text(&m.content)],
            ));
        }
        let input = ChatInput::new(messages);

        // Start the model stream
        let token_stream: BoxStream<'static, Result<StreamChunk, OpenAiModelError>> =
            self.model.stream(&(), input).await.map_err(|e| {
                AgentChatError(format!("Failed to start stream: {e}"))
            })?;

        // Create the workflow run
        let run = Arc::new(Mutex::new(WorkflowRun::new()));
        {
            let mut runs = self.active_runs.write().await;
            runs.insert(session.id.clone(), Arc::clone(&run));
        }

        // Spawn a background task to drain the stream
        let session_id = session.id.clone();
        let active_runs = Arc::clone(&self.active_runs);

        // We need to save the assistant response back to the session file
        // once the stream is done.
        let store_path = {
            let store = session_store.read().await;
            store.session_path(&session_id)
        };

        tokio::spawn(async move {
            use futures::StreamExt as _;
            let mut stream = token_stream;
            let mut full_response = String::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        let is_done = chunk.done;
                        full_response.push_str(&chunk.delta);
                        {
                            let mut r = run.lock().await;
                            r.tokens.push(chunk);
                            if is_done {
                                r.done = true;
                            }
                        }
                        if is_done {
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Stream error for session {session_id}: {e}");
                        let mut r = run.lock().await;
                        r.tokens.push(StreamChunk {
                            delta: format!("\n[Error: {e}]"),
                            done: true,
                        });
                        r.done = true;
                        break;
                    }
                }
            }

            // Mark complete if the stream ended without a done flag
            {
                let mut r = run.lock().await;
                if !r.done {
                    r.tokens.push(StreamChunk {
                        delta: String::new(),
                        done: true,
                    });
                    r.done = true;
                }
            }

            // Persist the assistant reply to the session file
            if !full_response.is_empty() {
                if let Ok(data) = tokio::fs::read_to_string(&store_path).await {
                    if let Ok(mut session) = serde_json::from_str::<FileSession>(&data) {
                        session.messages.push(SerializableMessage {
                            role: "assistant".to_string(),
                            content: full_response,
                        });
                        if let Ok(json) = serde_json::to_string_pretty(&session) {
                            let _ = tokio::fs::write(&store_path, json).await;
                        }
                    }
                }
            }

            // Clean up the run after a brief delay (allow clients to finish reading)
            tokio::time::sleep(std::time::Duration::from_secs(RUN_CLEANUP_DELAY_SECS)).await;
            {
                let mut runs = active_runs.write().await;
                runs.remove(&session_id);
            }
        });

        Ok(())
    }
}

/// Error type for chat handler operations.
#[derive(Debug)]
pub struct AgentChatError(pub String);

impl std::fmt::Display for AgentChatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chat handler error: {}", self.0)
    }
}

impl std::error::Error for AgentChatError {}

impl ChatHandler for AgentChatHandler {
    type Error = AgentChatError;
    type ResponseStream = BoxStream<'static, Result<StreamChunk, AgentChatError>>;

    fn chat<'a>(
        &'a self,
        session_id: &'a str,
        _message: &'a str,
    ) -> impl std::future::Future<Output = Result<Self::ResponseStream, Self::Error>> + Send + 'a
    {
        use futures::StreamExt;

        async move {
            // Look up the active run for this session
            let run = {
                let runs = self.active_runs.read().await;
                runs.get(session_id).cloned()
            };

            let run = run.ok_or_else(|| {
                AgentChatError(format!("No active run for session {session_id}"))
            })?;

            // Create a stream that replays buffered tokens and then yields new ones
            let (tx, rx) = tokio::sync::mpsc::channel::<Result<StreamChunk, AgentChatError>>(64);

            tokio::spawn(async move {
                let mut cursor = 0usize;
                loop {
                    let (chunks_to_send, is_done) = {
                        let r = run.lock().await;
                        let new_chunks: Vec<_> = r.tokens[cursor..].to_vec();
                        (new_chunks, r.done)
                    };

                    for chunk in &chunks_to_send {
                        if tx.send(Ok(chunk.clone())).await.is_err() {
                            return; // receiver dropped (client disconnected)
                        }
                        cursor += 1;
                    }

                    if is_done && chunks_to_send.is_empty() {
                        break;
                    }

                    tokio::time::sleep(std::time::Duration::from_millis(SSE_POLL_INTERVAL_MS)).await;
                }
            });

            let stream = tokio_stream::wrappers::ReceiverStream::new(rx).boxed();
            Ok(stream)
        }
    }
}
