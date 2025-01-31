use crate::traits::ActionSelector;

/// Type alias for an action selector factory.
pub type ActionSelectorFactory = Box<dyn Fn() -> Box<dyn ActionSelector + Send> + Send + Sync>;
