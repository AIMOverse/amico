use anyhow::Result;
use mcp_core::{
    server::Server,
    tool_text_content,
    transport::ServerSseTransport,
    types::{ServerCapabilities, ToolResponseContent},
};
use mcp_core_macros::tool;
use serde_json::json;
use tokio::task::JoinHandle;

#[tool(
    name = "Add",
    description = "Adds two numbers together.",
    params(a = "The first number to add", b = "The second number to add")
)]
pub async fn add_tool(a: f64, b: f64) -> Result<ToolResponseContent> {
    Ok(tool_text_content!((a + b).to_string()))
}

/// Start a local MCP server for testing
///
/// Returns a JoinHandle that can be used to shut down the server after tests
pub fn start_test_server(host: String, port: u16) -> JoinHandle<Result<()>> {
    // Create the MCP server
    let mcp_server_protocol = Server::builder("add".to_string(), "1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({
                "listChanged": false,
            })),
            ..Default::default()
        })
        .register_tool(AddTool::tool(), AddTool::call())
        .build();
    let mcp_server_transport = ServerSseTransport::new(host, port, mcp_server_protocol);
    let handle = tokio::spawn(async move { Server::start(mcp_server_transport).await });

    handle
}
