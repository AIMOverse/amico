//! Plugins actually provide the AI agent with functionality.

mod builder;
mod error;
mod interface;
mod pool;

#[cfg(test)]
mod tests;

pub use interface::*;
pub use pool::*;
