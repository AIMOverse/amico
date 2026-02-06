//! Application state and logic for the TUI.

use crate::api::{ApiClient, MessageDto, SessionDto, StreamEvent};
use futures::StreamExt;
use tokio::sync::mpsc;

/// Which panel is currently focused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Sessions,
    Chat,
}

/// Application state.
pub struct App {
    pub client: ApiClient,

    // -- sessions --
    pub sessions: Vec<SessionDto>,
    pub selected_session: usize,
    pub current_session_id: Option<String>,

    // -- chat --
    pub messages: Vec<MessageDto>,
    pub input: String,
    pub streaming_text: String,
    pub is_streaming: bool,

    // -- focus --
    pub focus: Focus,

    // -- SSE receiver --
    sse_rx: Option<mpsc::Receiver<StreamEvent>>,

    // -- status bar --
    pub status: String,
}

impl App {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: ApiClient::new(base_url),
            sessions: Vec::new(),
            selected_session: 0,
            current_session_id: None,
            messages: Vec::new(),
            input: String::new(),
            streaming_text: String::new(),
            is_streaming: false,
            focus: Focus::Sessions,
            sse_rx: None,
            status: "Press Ctrl+N to create a session, Tab to switch panels, Esc to quit"
                .to_string(),
        }
    }

    pub fn is_chat_focused(&self) -> bool {
        self.focus == Focus::Chat
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Sessions => Focus::Chat,
            Focus::Chat => Focus::Sessions,
        };
    }

    pub fn on_up(&mut self) {
        if self.focus == Focus::Sessions && self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }

    pub fn on_down(&mut self) {
        if self.focus == Focus::Sessions && self.selected_session + 1 < self.sessions.len() {
            self.selected_session += 1;
        }
    }

    pub async fn on_enter(&mut self) {
        match self.focus {
            Focus::Sessions => {
                // Select the highlighted session
                if let Some(session) = self.sessions.get(self.selected_session) {
                    self.load_session(&session.id.clone()).await;
                }
            }
            Focus::Chat => {
                // Send the current input as a message
                if !self.input.is_empty() {
                    self.send_message().await;
                }
            }
        }
    }

    pub async fn refresh_sessions(&mut self) {
        match self.client.list_sessions().await {
            Ok(sessions) => {
                self.sessions = sessions;
                if self.selected_session >= self.sessions.len() {
                    self.selected_session = self.sessions.len().saturating_sub(1);
                }
            }
            Err(e) => {
                self.status = format!("Failed to list sessions: {e}");
            }
        }
    }

    pub async fn create_session(&mut self) {
        match self.client.create_session().await {
            Ok(session) => {
                self.status = format!("Created session {}", &session.id[..8]);
                let id = session.id.clone();
                self.sessions.push(session);
                self.selected_session = self.sessions.len() - 1;
                self.load_session(&id).await;
            }
            Err(e) => {
                self.status = format!("Failed to create session: {e}");
            }
        }
    }

    async fn load_session(&mut self, id: &str) {
        match self.client.get_session(id).await {
            Ok(detail) => {
                self.current_session_id = Some(detail.id.clone());
                self.messages = detail.messages;
                self.streaming_text.clear();
                self.is_streaming = false;
                self.sse_rx = None;
                self.status = format!("Session {}", &detail.id[..8.min(detail.id.len())]);

                // If the session has an active run, reconnect to its stream
                if detail.has_active_run {
                    self.connect_stream(id).await;
                }
            }
            Err(e) => {
                self.status = format!("Failed to load session: {e}");
            }
        }
    }

    async fn send_message(&mut self) {
        let content = self.input.clone();
        self.input.clear();

        let session_id = match &self.current_session_id {
            Some(id) => id.clone(),
            None => {
                self.status = "No session selected".to_string();
                return;
            }
        };

        // Add user message to local display immediately
        self.messages.push(MessageDto {
            role: "user".to_string(),
            content: content.clone(),
        });

        // Send to daemon
        if let Err(e) = self.client.send_message(&session_id, &content).await {
            self.status = format!("Failed to send: {e}");
            return;
        }

        self.status = "Waiting for response...".to_string();
        self.streaming_text.clear();
        self.is_streaming = true;

        // Connect to SSE stream
        self.connect_stream(&session_id).await;
    }

    async fn connect_stream(&mut self, session_id: &str) {
        let (tx, rx) = mpsc::channel::<StreamEvent>(256);
        self.sse_rx = Some(rx);
        self.is_streaming = true;

        let client = ApiClient::new(&format!(
            "{}",
            std::env::var("AGENT_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into())
        ));
        let sid = session_id.to_string();

        tokio::spawn(async move {
            // Small delay to let the daemon start the workflow
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

            let resp = match client.stream_session(&sid).await {
                Ok(r) => r,
                Err(_) => return,
            };

            let mut byte_stream = resp.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk_result) = byte_stream.next().await {
                match chunk_result {
                    Ok(bytes) => {
                        buffer.push_str(&String::from_utf8_lossy(&bytes));

                        // Process complete SSE lines
                        while let Some(line_end) = buffer.find('\n') {
                            let line = buffer[..line_end].trim().to_string();
                            buffer = buffer[line_end + 1..].to_string();

                            if let Some(data) = line.strip_prefix("data:") {
                                let data = data.trim();
                                if let Ok(event) =
                                    serde_json::from_str::<StreamEvent>(data)
                                {
                                    let is_done =
                                        event.done.unwrap_or(false) || event.error.is_some();
                                    if tx.send(event).await.is_err() {
                                        return;
                                    }
                                    if is_done {
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    /// Poll the SSE receiver for new tokens (non-blocking).
    pub async fn poll_stream(&mut self) {
        if let Some(rx) = &mut self.sse_rx {
            // Drain all available events
            loop {
                match rx.try_recv() {
                    Ok(event) => {
                        if let Some(delta) = &event.delta {
                            self.streaming_text.push_str(delta);
                        }
                        if let Some(error) = &event.error {
                            self.status = format!("Error: {error}");
                            self.is_streaming = false;
                        }
                        if event.done.unwrap_or(false) {
                            // Streaming complete — commit to messages
                            if !self.streaming_text.is_empty() {
                                self.messages.push(MessageDto {
                                    role: "assistant".to_string(),
                                    content: self.streaming_text.clone(),
                                });
                                self.streaming_text.clear();
                            }
                            self.is_streaming = false;
                            self.sse_rx = None;
                            self.status = "Ready".to_string();
                            break;
                        }
                    }
                    Err(mpsc::error::TryRecvError::Empty) => break,
                    Err(mpsc::error::TryRecvError::Disconnected) => {
                        // Stream ended unexpectedly
                        if !self.streaming_text.is_empty() {
                            self.messages.push(MessageDto {
                                role: "assistant".to_string(),
                                content: self.streaming_text.clone(),
                            });
                            self.streaming_text.clear();
                        }
                        self.is_streaming = false;
                        self.sse_rx = None;
                        self.status = "Stream ended".to_string();
                        break;
                    }
                }
            }
        }
    }
}
