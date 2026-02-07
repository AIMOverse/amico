//! Execution context for workflows.

/// Execution context for workflows.
///
/// Provides access to the shared state and permission set that a
/// workflow needs during execution.
pub trait ExecutionContext {
    /// State type managed by the context
    type State;

    /// Permission type for resource access
    type Permissions;

    /// Get immutable reference to state
    fn state(&self) -> &Self::State;

    /// Get mutable reference to state
    fn state_mut(&mut self) -> &mut Self::State;

    /// Get permissions
    fn permissions(&self) -> &Self::Permissions;
}

/// Simple execution context implementation.
#[derive(Debug)]
pub struct SimpleContext<S, P> {
    state: S,
    permissions: P,
}

impl<S, P> SimpleContext<S, P> {
    pub fn new(state: S, permissions: P) -> Self {
        Self { state, permissions }
    }
}

impl<S, P> ExecutionContext for SimpleContext<S, P> {
    type State = S;
    type Permissions = P;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }

    fn permissions(&self) -> &Self::Permissions {
        &self.permissions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_context() {
        let mut ctx = SimpleContext::new(42u32, "read-only");
        assert_eq!(*ctx.state(), 42);
        assert_eq!(*ctx.permissions(), "read-only");

        *ctx.state_mut() = 100;
        assert_eq!(*ctx.state(), 100);
    }
}
