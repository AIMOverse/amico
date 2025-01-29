use std::any::Any;

/// Trait representing an action in the system.
pub trait Action {
    /// Executes the action and returns a response as a boxed `Any` type.
    fn execute(&self) -> Box<dyn Any>;
}
