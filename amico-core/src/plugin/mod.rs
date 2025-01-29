//! Plugins actually provide the AI agent with functionality.
//!
//! ## The Plugin Struct
//!
//! This is what a plugin module provides and can be 'plugged'
//! into an AI agent runtime.
//!
//! ## Plugin Components
//!
//! Each plugin may provide multiple components to use in
//! different steps of the agent workflow.
//!

mod interface;
mod pool;

pub use interface::*;
pub use pool::*;
