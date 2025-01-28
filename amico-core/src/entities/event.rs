/// Trait representing an event in the system.
pub trait Event {
    /// Returns the name of the event.
    fn name(&self) -> &str;

    /// Returns the source of the event.
    fn source(&self) -> &str;

    /// Returns the parameters of the event.
    fn params(&self) -> &str;
}