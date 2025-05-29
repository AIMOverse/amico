use std::fmt::Debug;

/// Errors that can occur in event pool.
#[derive(thiserror::Error, Debug)]
pub enum EventPoolError {
    /// Error when the there is no available event IDs left.
    #[error("No available event IDs left")]
    NoAvailableEventIds,
    /// Error when No such event ID.
    #[error("Event ID not found: {0}")]
    EventIdNotFound(u32),

    /// Error when the event IDs are not found.
    #[error("Event IDs not found: {0:?}")]
    SomeEventIdsNotFound(Vec<u32>),
}
