use tokio::task::JoinHandle;
use tokio_with_wasm::alias as tokio;

use crate::types::AgentEvent;

/// An event source listens for events, and calls the `on_event`
/// callback to react to events.
pub trait EventSource {
    /// The method to run the `EventSource`.
    ///
    /// The `run` method will be called by `Agent` in a new thread.
    fn spawn<F, Fut>(&self, on_event: F) -> JoinHandle<anyhow::Result<()>>
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static;
}
