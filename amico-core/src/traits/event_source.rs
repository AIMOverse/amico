use std::future::Future;

use crate::types::AgentEvent;

/// An event source listens for events, and calls the `on_event`
/// callback to react to events.
pub trait EventSource {
    /// The method to run the `EventSource`.
    ///
    /// ## Notice
    ///
    /// The `run` method **SHOULD NOT SPAWN A THREAD**.
    /// Event source threads should be handled by `Agent`.
    ///
    /// The `run` method only cares about when to call the
    /// `on_event` callback.
    fn run<F, Fut>(&self, on_event: F) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: Fn(AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static;
}
