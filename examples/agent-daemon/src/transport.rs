//! HTTP transport — serves the agent API over HTTP with SSE streaming.
//!
//! This is one possible `amico::Transport` implementation. Other transports
//! (WebSocket, gRPC, stdin/stdout) can be added without changing the agent
//! logic.
//!
//! ## Endpoints
//!
//! | Method | Path                         | Description                          |
//! |--------|------------------------------|--------------------------------------|
//! | GET    | `/api/sessions`              | List all sessions                    |
//! | POST   | `/api/sessions`              | Create a new session                 |
//! | GET    | `/api/sessions/:id`          | Get session details                  |
//! | DELETE | `/api/sessions/:id`          | Delete a session                     |
//! | POST   | `/api/sessions/:id/messages` | Send a message (starts workflow)     |
//! | GET    | `/api/sessions/:id/stream`   | SSE stream of the active workflow    |

use crate::handler::AgentChatError;
use crate::AppState;
use amico::ChatHandler;
use amico_models::StreamChunk;
use amico_runtime::fs_store::SerializableMessage;
use amico_runtime::SessionStore;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, Sse},
        Json,
    },
    routing::{get, post},
    Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

// -- DTO types for the API --

#[derive(Serialize)]
struct SessionDto {
    id: String,
    created_at: u64,
    message_count: usize,
}

#[derive(Serialize)]
struct SessionDetailDto {
    id: String,
    created_at: u64,
    messages: Vec<MessageDto>,
    has_active_run: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct MessageDto {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct SendMessageRequest {
    content: String,
}

#[derive(Serialize)]
struct ErrorDto {
    error: String,
}

// -- Route handlers --

async fn list_sessions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SessionDto>>, (StatusCode, Json<ErrorDto>)> {
    let store = state.session_store.read().await;
    let sessions = store.list_sessions().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;
    let dtos = sessions
        .iter()
        .map(|s| SessionDto {
            id: s.id.clone(),
            created_at: s.created_at,
            message_count: s.messages.len(),
        })
        .collect();
    Ok(Json(dtos))
}

async fn create_session(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<SessionDto>), (StatusCode, Json<ErrorDto>)> {
    let store = state.session_store.read().await;
    let session = store.create_session().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;
    Ok((
        StatusCode::CREATED,
        Json(SessionDto {
            id: session.id,
            created_at: session.created_at,
            message_count: 0,
        }),
    ))
}

async fn get_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<SessionDetailDto>, (StatusCode, Json<ErrorDto>)> {
    let store = state.session_store.read().await;
    let session = store.get_session(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;
    let session = session.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorDto {
                error: "Session not found".into(),
            }),
        )
    })?;
    let has_active_run = {
        let runs = state.chat_handler.active_runs.read().await;
        runs.contains_key(&session.id)
    };
    Ok(Json(SessionDetailDto {
        id: session.id,
        created_at: session.created_at,
        messages: session
            .messages
            .iter()
            .map(|m| MessageDto {
                role: m.role.clone(),
                content: m.content.clone(),
            })
            .collect(),
        has_active_run,
    }))
}

async fn delete_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorDto>)> {
    let store = state.session_store.read().await;
    store.delete_session(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;
    Ok(StatusCode::NO_CONTENT)
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<SendMessageRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorDto>)> {
    // Load the session
    let store = state.session_store.read().await;
    let session = store.get_session(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;
    let mut session = session.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorDto {
                error: "Session not found".into(),
            }),
        )
    })?;

    // Append user message
    session.messages.push(SerializableMessage {
        role: "user".to_string(),
        content: body.content,
    });
    store.save_session(&session).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;

    // Start background workflow
    state
        .chat_handler
        .start_workflow(&session, &state.session_store)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorDto {
                    error: e.to_string(),
                }),
            )
        })?;

    Ok(StatusCode::ACCEPTED)
}

/// SSE endpoint — streams the active workflow run for a session.
///
/// If the client reconnects while a workflow is still running, it gets
/// all previously buffered tokens followed by live tokens (resume).
async fn stream_session(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, (StatusCode, Json<ErrorDto>)>
{
    let response_stream = state.chat_handler.chat(&id, "").await.map_err(|e| {
        (
            StatusCode::NOT_FOUND,
            Json(ErrorDto {
                error: e.to_string(),
            }),
        )
    })?;

    let event_stream = response_stream.map(|result: Result<StreamChunk, AgentChatError>| {
        match result {
            Ok(chunk) => {
                let data = serde_json::json!({
                    "delta": chunk.delta,
                    "done": chunk.done,
                });
                Ok(Event::default().data(data.to_string()))
            }
            Err(e) => {
                let data = serde_json::json!({
                    "error": e.to_string(),
                    "done": true,
                });
                Ok(Event::default().data(data.to_string()))
            }
        }
    });

    Ok(Sse::new(event_stream))
}

// -- Router & server --

fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/sessions", get(list_sessions).post(create_session))
        .route(
            "/api/sessions/{id}",
            get(get_session).delete(delete_session),
        )
        .route("/api/sessions/{id}/messages", post(send_message))
        .route("/api/sessions/{id}/stream", get(stream_session))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start the HTTP transport server.
pub async fn serve(state: Arc<AppState>, bind_addr: &str) -> crate::anyhow::Result<()> {
    let app = build_router(state);
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|e| -> crate::anyhow::Error { e.into() })?;
    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| -> crate::anyhow::Error { e.into() })?;
    Ok(())
}

// -- Transport trait implementation (informational) --
// The `serve` function above is the concrete implementation.
// A formal `impl amico::Transport for HttpTransport` can wrap it when
// needed; keeping it as a free function keeps the example concise.
