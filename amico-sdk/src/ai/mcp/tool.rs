//! Re-export MCP tool components from `rig-core`

use super::McpTransport;

/// MCP tool
pub type McpTool = rig::tool::McpTool<McpTransport>;

/// MCP tool error
pub type McpToolError = rig::tool::McpToolError;
