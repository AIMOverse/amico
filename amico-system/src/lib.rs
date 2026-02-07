//! # Amico System Layer
//!
//! This crate provides the system layer for Amico V2, which defines how agents
//! interact with the real world through tools and side-effects.
//!
//! ## Design Principles
//!
//! - **Platform abstraction**: Adapts to different platforms (OS, browser, mobile, embedded)
//! - **Permission model**: Secure, permission-based access to system resources
//! - **Tool abstraction**: Unified interface for all tools
//! - **Event streams**: Observable patterns for sensors and environmental changes
//! - **Side-effect isolation**: Using a unified system interface makes building
//!   cross-platform agents easy. The system may change the underlying implementation
//!   without modifying agent logic, and helps identify platform-incompatible usage
//!   at compile time.
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico_system::{Tool, SystemEffect};
//!
//! struct FileTool;
//!
//! impl Tool for FileTool {
//!     type Input = String;
//!     type Output = String;
//!     type Error = std::io::Error;
//!
//!     async fn execute(&self, path: String) -> Result<String, Self::Error> {
//!         tokio::fs::read_to_string(path).await
//!     }
//!
//!     fn name(&self) -> &str { "read_file" }
//!     fn description(&self) -> &str { "Read contents of a file" }
//! }
//! ```

pub mod effect;
pub mod permission;
pub mod platform;
pub mod stream;
pub mod tool;

// Re-export all public items for backward compatibility
pub use effect::SystemEffect;
pub use permission::{Permission, PermissionChecker, ResourcePermission};
pub use platform::{
    Clock, Entropy, FileOperation, FileResult, LogLevel, Logger, NetworkOperation, NetworkResult,
    ProcessOperation, ProcessResult, System,
};
pub use stream::{Observable, Stream};
pub use tool::Tool;
