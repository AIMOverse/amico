/// Trait representing an action in the system.
pub trait Action {
    /// Executes the action and returns a response as a string.
    fn execute(&self) -> String;
}