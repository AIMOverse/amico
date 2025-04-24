pub mod error;
mod interface;

pub use interface::A2aNetwork;

#[cfg(feature = "a2a-dephy")]
pub mod dephy;
