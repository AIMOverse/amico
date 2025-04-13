//! Re-export MCP tool components from `rig-core`

use rig::tool::ToolDyn;
use std::str::FromStr;
use std::sync::Arc;

use crate::ai::{
    errors::ToolCallError,
    tool::{Tool, ToolBuilder},
};

use super::{McpClient, McpTransport};

/// MCP tool
pub struct McpTool {
    name: String,
    description: Option<String>,
    params: serde_json::Value,
    mcp_tool: rig::tool::McpTool<McpTransport>,
}

impl McpTool {
    /// Build the MCP tool instance from MCP Client.
    pub fn from_mcp_server(definition: mcp_core::types::Tool, client: McpClient) -> Self {
        Self {
            name: definition.name.clone(),
            description: definition.description.clone(),
            params: definition.input_schema.clone(),
            mcp_tool: rig::tool::McpTool::from_mcp_server(definition, client),
        }
    }
}

impl From<McpTool> for Tool {
    /// Convert the MCP tool to a `Tool` instance
    fn from(val: McpTool) -> Self {
        // Wrap mcp_tool in an Arc to share ownership across async calls
        let mcp_tool = Arc::new(val.mcp_tool);

        ToolBuilder::new()
            .name(&val.name)
            .description(&val.description.unwrap_or("".to_string()))
            .parameters(val.params)
            .build_async(move |args| {
                let args = args.clone();
                let args_str = args.to_string();
                let name = val.name.clone();
                let mcp_tool = mcp_tool.clone(); // Clone the Arc, not the inner value

                async move {
                    mcp_tool
                        .call(args_str)
                        .await
                        .map(|res| {
                            serde_json::Value::from_str(&res).unwrap_or(serde_json::json!(res))
                        })
                        .map_err(|err| ToolCallError::ExecutionError {
                            tool_name: name,
                            params: args,
                            reason: err.to_string(),
                        })
                }
            })
    }
}
