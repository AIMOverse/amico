//! Filesystem-backed session store.
//!
//! Session storage is an application concern — the framework provides the
//! `SessionStore` trait and lifecycle hooks, while the developer implements
//! actual persistence. Using the `amico_system` FS interface would enable
//! cross-platform agents in the future.

use amico_runtime::{Session, SessionStore};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::path::PathBuf;

/// A chat session stored on the local filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSession {
    pub id: String,
    pub created_at: u64,
    pub messages: Vec<SerializableMessage>,
}

/// Chat message in a serializable form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMessage {
    pub role: String,
    pub content: String,
}

impl Session for FileSession {
    fn id(&self) -> &str {
        &self.id
    }

    fn created_at(&self) -> u64 {
        self.created_at
    }
}

/// Error type for the file session store.
#[derive(Debug)]
pub struct FileSessionError(pub String);

impl std::fmt::Display for FileSessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session store error: {}", self.0)
    }
}

impl std::error::Error for FileSessionError {}

/// Stores sessions as JSON files under `data_dir/<session_id>.json`.
pub struct FileSessionStore {
    data_dir: PathBuf,
}

impl FileSessionStore {
    pub async fn new(data_dir: &str) -> Result<Self, FileSessionError> {
        let path = PathBuf::from(data_dir);
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|e| FileSessionError(e.to_string()))?;
        Ok(Self { data_dir: path })
    }

    pub fn session_path(&self, id: &str) -> PathBuf {
        self.data_dir.join(format!("{id}.json"))
    }
}

impl SessionStore for FileSessionStore {
    type Session = FileSession;
    type Error = FileSessionError;

    fn create_session(
        &self,
    ) -> impl Future<Output = Result<Self::Session, Self::Error>> + Send {
        async move {
            let session = FileSession {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
                messages: Vec::new(),
            };
            let json = serde_json::to_string_pretty(&session)
                .map_err(|e| FileSessionError(e.to_string()))?;
            tokio::fs::write(self.session_path(&session.id), json)
                .await
                .map_err(|e| FileSessionError(e.to_string()))?;
            Ok(session)
        }
    }

    fn get_session(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Self::Session>, Self::Error>> + Send {
        let path = self.session_path(id);
        async move {
            if !path.exists() {
                return Ok(None);
            }
            let data = tokio::fs::read_to_string(&path)
                .await
                .map_err(|e| FileSessionError(e.to_string()))?;
            let session: FileSession =
                serde_json::from_str(&data).map_err(|e| FileSessionError(e.to_string()))?;
            Ok(Some(session))
        }
    }

    fn save_session(
        &self,
        session: &Self::Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let path = self.session_path(&session.id);
        let json_result = serde_json::to_string_pretty(session);
        async move {
            let json = json_result.map_err(|e| FileSessionError(e.to_string()))?;
            tokio::fs::write(path, json)
                .await
                .map_err(|e| FileSessionError(e.to_string()))?;
            Ok(())
        }
    }

    fn list_sessions(
        &self,
    ) -> impl Future<Output = Result<Vec<Self::Session>, Self::Error>> + Send {
        let dir = self.data_dir.clone();
        async move {
            let mut sessions = Vec::new();
            let mut entries = tokio::fs::read_dir(&dir)
                .await
                .map_err(|e| FileSessionError(e.to_string()))?;
            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| FileSessionError(e.to_string()))?
            {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    let data = tokio::fs::read_to_string(&path)
                        .await
                        .map_err(|e| FileSessionError(e.to_string()))?;
                    if let Ok(session) = serde_json::from_str::<FileSession>(&data) {
                        sessions.push(session);
                    }
                }
            }
            sessions.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            Ok(sessions)
        }
    }

    fn delete_session(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let path = self.session_path(id);
        async move {
            if path.exists() {
                tokio::fs::remove_file(path)
                    .await
                    .map_err(|e| FileSessionError(e.to_string()))?;
            }
            Ok(())
        }
    }
}
