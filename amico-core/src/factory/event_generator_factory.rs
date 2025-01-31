use crate::traits::EventGenerator;

/// Type alias for an event generator factory.
pub type EventGeneratorFactory = Box<dyn Fn() -> Box<dyn EventGenerator + Send> + Send + Sync>;
