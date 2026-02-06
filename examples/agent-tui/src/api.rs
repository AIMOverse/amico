//! HTTP API client for the agent daemon.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDto {
    pub id: String,
    pub created_at: u64,
    pub message_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionDetailDto {
    pub id: String,
    pub created_at: u64,
    pub messages: Vec<MessageDto>,
    pub has_active_run: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDto {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    pub delta: Option<String>,
    pub done: Option<bool>,
    pub error: Option<String>,
}

/// API client for the agent daemon.
pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionDto>, String> {
        let resp = self
            .client
            .get(format!("{}/api/sessions", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn create_session(&self) -> Result<SessionDto, String> {
        let resp = self
            .client
            .post(format!("{}/api/sessions", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn get_session(&self, id: &str) -> Result<SessionDetailDto, String> {
        let resp = self
            .client
            .get(format!("{}/api/sessions/{id}", self.base_url))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    pub async fn send_message(&self, session_id: &str, content: &str) -> Result<(), String> {
        let body = serde_json::json!({ "content": content });
        let resp = self
            .client
            .post(format!(
                "{}/api/sessions/{session_id}/messages",
                self.base_url
            ))
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(format!("HTTP {}", resp.status()))
        }
    }

    /// Start reading the SSE stream for a session.
    /// Returns the raw byte stream for line-by-line parsing.
    pub async fn stream_session(
        &self,
        session_id: &str,
    ) -> Result<reqwest::Response, String> {
        self.client
            .get(format!(
                "{}/api/sessions/{session_id}/stream",
                self.base_url
            ))
            .send()
            .await
            .map_err(|e| e.to_string())
    }
}
