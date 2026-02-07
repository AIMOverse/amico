//! Side-effect abstraction for modifying system state.

use std::future::Future;

/// System effect - represents a side effect that modifies system state.
///
/// Side effects are explicit, typed operations that change the world
/// outside the agent's pure logic. By isolating them behind a trait,
/// the same agent logic can run on different platforms.
pub trait SystemEffect {
    /// The system state being modified
    type State;

    /// Action to apply to the state
    type Action;

    /// Result of applying the action
    type Result;

    /// Error type
    type Error;

    /// Apply an action to modify system state
    fn apply<'a>(
        &'a mut self,
        action: Self::Action,
    ) -> impl Future<Output = Result<Self::Result, Self::Error>> + Send + 'a;
}
