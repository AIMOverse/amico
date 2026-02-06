//! Shell tool — executes commands on behalf of the agent.
//!
//! This is an example of a tool written by the agent developer using the
//! `amico_system::Tool` trait. The framework does not provide concrete tools —
//! developers build tools using the system APIs, enabling cross-platform
//! compilation and permission checking.

use amico_system::Tool;
use std::future::Future;

/// A tool that executes a shell command and returns its output.
pub struct ShellTool;

/// Input for the shell tool.
#[derive(Debug, Clone)]
pub struct ShellInput {
    pub command: String,
}

/// Output from the shell tool.
#[derive(Debug, Clone)]
pub struct ShellOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Error type for the shell tool.
#[derive(Debug)]
pub struct ShellToolError(pub String);

impl std::fmt::Display for ShellToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shell tool error: {}", self.0)
    }
}

impl std::error::Error for ShellToolError {}

impl Tool for ShellTool {
    type Input = ShellInput;
    type Output = ShellOutput;
    type Error = ShellToolError;

    fn execute<'a>(
        &'a self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a {
        async move {
            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&input.command)
                .output()
                .await
                .map_err(|e| ShellToolError(e.to_string()))?;

            Ok(ShellOutput {
                exit_code: output.status.code().unwrap_or(-1),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }
    }

    fn name(&self) -> &str {
        "shell"
    }

    fn description(&self) -> &str {
        "Execute a shell command and return its output"
    }
}
