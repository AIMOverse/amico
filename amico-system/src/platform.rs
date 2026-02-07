//! Platform abstraction traits.
//!
//! These traits define platform-specific capabilities that the agent
//! runtime relies on. By coding against these traits (rather than
//! concrete implementations), agent logic becomes portable across
//! operating systems, browsers, mobile devices, and embedded targets.

use crate::Tool;
use std::future::Future;

/// A clock provides time-related operations.
///
/// Platform-specific implementations supply the underlying time source.
/// For example, `std::time` on native targets, `performance.now()` in
/// browsers, or a monotonic counter on embedded devices.
pub trait Clock {
    /// Returns the current timestamp in milliseconds since epoch.
    fn now_millis(&self) -> u64;
}

/// Log severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// A structured logger for agent lifecycle and diagnostic messages.
///
/// Implementations may write to stdout, a file, a remote logging
/// service, or simply discard messages depending on the platform.
pub trait Logger {
    type Error;

    /// Emit a log message at the given severity level.
    fn log<'a>(
        &'a self,
        level: LogLevel,
        message: &'a str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'a;
}

/// A source of randomness.
///
/// Agent logic that needs non-deterministic behaviour (e.g. sampling
/// temperatures, generating IDs) should use this trait instead of
/// calling platform-specific random APIs directly.
pub trait Entropy {
    /// Fill the provided buffer with random bytes.
    fn fill_bytes(&self, buf: &mut [u8]);
}

/// Platform-specific system interface.
///
/// The `System` trait bundles all platform capabilities that tools and
/// side effects may depend on. Swapping the `System` implementation
/// is the primary mechanism for porting an agent to a new platform.
pub trait System {
    /// File operations tool
    type FileOps: Tool;

    /// Network operations tool
    type NetworkOps: Tool;

    /// Process operations tool
    type ProcessOps: Tool;

    /// Platform clock
    type Clock: Clock;

    /// Platform logger
    type Logger: Logger;

    /// Platform entropy source
    type Entropy: Entropy;

    fn file_ops(&self) -> &Self::FileOps;
    fn network_ops(&self) -> &Self::NetworkOps;
    fn process_ops(&self) -> &Self::ProcessOps;
    fn clock(&self) -> &Self::Clock;
    fn logger(&self) -> &Self::Logger;
    fn entropy(&self) -> &Self::Entropy;
}

/// File system operations
#[derive(Debug, Clone)]
pub enum FileOperation {
    Read { path: String },
    Write { path: String, content: Vec<u8> },
    Delete { path: String },
    List { path: String },
}

/// File system result
#[derive(Debug, Clone)]
pub enum FileResult {
    Content(Vec<u8>),
    Success,
    Listing(Vec<String>),
}

/// Network operations
#[derive(Debug, Clone)]
pub enum NetworkOperation {
    HttpRequest {
        method: String,
        url: String,
        headers: Vec<(String, String)>,
        body: Option<Vec<u8>>,
    },
}

/// Network result
#[derive(Debug, Clone)]
pub struct NetworkResult {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Process operations
#[derive(Debug, Clone)]
pub enum ProcessOperation {
    Execute {
        command: String,
        args: Vec<String>,
        env: Vec<(String, String)>,
    },
}

/// Process result
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub exit_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}
