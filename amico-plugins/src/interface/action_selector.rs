use crate::interface::Plugin;
use amico_core::traits::ActionSelector;

/// The trait for ActionSelector Plugins
pub trait ActionSelectorPlugin: Plugin + ActionSelector {}
