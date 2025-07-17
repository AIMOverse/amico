//! Lightweight event system for amico-core
//! 
//! This module provides a no-std compatible, async-friendly event system
//! with minimal heap allocation.

mod bus;
mod handler;
mod types;

pub use bus::*;
pub use handler::*;
pub use types::*;