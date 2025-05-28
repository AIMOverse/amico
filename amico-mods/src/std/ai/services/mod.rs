#[cfg(feature = "std-services-in-memory")]
mod in_memory;
#[cfg(feature = "std-services-in-memory")]
pub use in_memory::*;

pub mod speech;
