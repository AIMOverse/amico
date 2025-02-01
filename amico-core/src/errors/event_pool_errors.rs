use std::fmt::Debug;

/// Errors that can occur in event pool.
#[derive(thiserror::Error, Debug)]
pub enum EventPoolError {
    /// Error when the there is no available event IDs left.
    #[error("No available event IDs left")]
    NoAvailableEventIds,
}
