//! Stream and observable abstractions for event-driven patterns.

/// Stream trait for observable events.
///
/// A lightweight, synchronous stream interface. Platform-specific
/// implementations can bridge to `futures::Stream` or other async
/// stream types.
pub trait Stream {
    type Item;

    /// Poll the next item from the stream
    fn poll_next(&mut self) -> Option<Self::Item>;
}

/// Observable stream of events (sensors, environmental changes).
///
/// An `Observable` produces a `Stream` that clients can subscribe to
/// in order to observe events as they occur.
pub trait Observable {
    /// Type of events emitted
    type Event;

    /// Event stream type
    type Stream: Stream<Item = Self::Event>;

    /// Subscribe to events from this observable
    fn subscribe(&self) -> Self::Stream;
}
