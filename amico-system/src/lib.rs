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

use std::future::Future;

// ============================================================
// Optional concrete implementations
// ============================================================

/// Shell tool for executing commands on the host OS.
///
/// Enable with the `shell` feature flag.
#[cfg(feature = "shell")]
pub mod shell;

/// Core tool trait - all tools implement this
pub trait Tool {
    /// Input type for the tool
    type Input;
    
    /// Output type produced by the tool
    type Output;
    
    /// Error type for tool execution
    type Error;

    /// Execute the tool with given input
    fn execute<'a>(
        &'a self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a;
    
    /// Tool name (used for identification)
    fn name(&self) -> &str;
    
    /// Human-readable description of what the tool does
    fn description(&self) -> &str;
    
    /// JSON schema for the tool's input (optional)
    fn input_schema(&self) -> Option<&str> {
        None
    }
}

/// System effect - represents a side effect that modifies system state
pub trait SystemEffect {
    /// The system state being modified
    type State;
    
    /// Action to apply to the state
    type Action;
    
    /// Result of applying the action
    type Result;
    
    /// Error type
    type Error;
    
    /// Apply an action to modify system state
    fn apply<'a>(
        &'a mut self,
        action: Self::Action,
    ) -> impl Future<Output = Result<Self::Result, Self::Error>> + Send + 'a;
}

/// Permission system for secure resource access
pub trait Permission<R> {
    /// Check if access to a resource is permitted
    fn check(&self, resource: &R) -> bool;
    
    /// Grant permission to access a resource
    fn grant(&mut self, resource: R);
    
    /// Revoke permission to access a resource
    fn revoke(&mut self, resource: &R);
}

/// Stream trait for observable events
pub trait Stream {
    type Item;
    
    /// Poll the next item from the stream
    fn poll_next(&mut self) -> Option<Self::Item>;
}

/// Observable stream of events (sensors, environmental changes)
pub trait Observable {
    /// Type of events emitted
    type Event;
    
    /// Event stream type
    type Stream: Stream<Item = Self::Event>;
    
    /// Subscribe to events from this observable
    fn subscribe(&self) -> Self::Stream;
}

/// Platform-specific system interface
pub trait System {
    /// File operations tool
    type FileOps: Tool;
    
    /// Network operations tool
    type NetworkOps: Tool;
    
    /// Process operations tool
    type ProcessOps: Tool;
    
    fn file_ops(&self) -> &Self::FileOps;
    fn network_ops(&self) -> &Self::NetworkOps;
    fn process_ops(&self) -> &Self::ProcessOps;
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

/// Permission types for system resources
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourcePermission {
    FileRead(String),
    FileWrite(String),
    NetworkAccess(String),
    ProcessExecution,
}

/// Simple permission checker
#[derive(Debug, Clone, Default)]
pub struct PermissionChecker {
    granted: Vec<ResourcePermission>,
}

impl PermissionChecker {
    pub fn new() -> Self {
        Self {
            granted: Vec::new(),
        }
    }
    
    pub fn grant_all(&mut self) {
        // Grant all permissions (use with caution!)
        self.granted.push(ResourcePermission::ProcessExecution);
    }
}

impl Permission<ResourcePermission> for PermissionChecker {
    fn check(&self, resource: &ResourcePermission) -> bool {
        self.granted.contains(resource)
    }
    
    fn grant(&mut self, resource: ResourcePermission) {
        if !self.granted.contains(&resource) {
            self.granted.push(resource);
        }
    }
    
    fn revoke(&mut self, resource: &ResourcePermission) {
        self.granted.retain(|r| r != resource);
    }
}
