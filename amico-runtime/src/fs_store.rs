//! Filesystem-backed session store.
//!
//! Implements [`SessionStore`](crate::SessionStore) by persisting each session
//! as a JSON file under a configurable directory.
//!
//! # Example
//!
//! ```rust,ignore
//! use amico_runtime::fs_store::FileSessionStore;
//! use amico_runtime::SessionStore;
//!
//! let store = FileSessionStore::new(".amico/sessions").await?;
//! let session = store.create_session().await?;
//! println!("Created session: {}", session.id);
//! ```

use crate::{Session, SessionStore};
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
///
/// This is a transport-neutral representation used for persistence.
/// Conversion to/from framework message types is handled by the
/// application layer.
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
    /// Create a new filesystem session store.
    ///
    /// Creates the directory if it doesn't exist.
    pub async fn new(data_dir: &str) -> Result<Self, FileSessionError> {
        let path = PathBuf::from(data_dir);
        tokio::fs::create_dir_all(&path)
            .await
            .map_err(|e| FileSessionError(e.to_string()))?;
        Ok(Self { data_dir: path })
    }

    /// Get the file path for a session by ID.
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
            // Persist immediately
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_crud() {
        let dir = std::env::temp_dir().join(format!("amico_test_{}", uuid::Uuid::new_v4()));
        let store = FileSessionStore::new(dir.to_str().unwrap()).await.unwrap();

        // Create
        let session = store.create_session().await.unwrap();
        assert!(!session.id.is_empty());

        // Get
        let fetched = store.get_session(&session.id).await.unwrap();
        assert!(fetched.is_some());
        assert_eq!(fetched.unwrap().id, session.id);

        // List
        let all = store.list_sessions().await.unwrap();
        assert_eq!(all.len(), 1);

        // Delete
        store.delete_session(&session.id).await.unwrap();
        let gone = store.get_session(&session.id).await.unwrap();
        assert!(gone.is_none());

        // Cleanup
        let _ = tokio::fs::remove_dir_all(&dir).await;
    }
}
