use crate::ai::mcp::{McpClientBuilder, McpTool, test_server::*};
use serde_json::{Value, json};

const TEST_HOST: &str = "127.0.0.1";
const TEST_PORT: u16 = 3000;
const TEST_URL: &str = "http://127.0.0.1:3000/sse";

/// Helper function to start the test server
async fn setup_test_server() -> tokio::task::JoinHandle<anyhow::Result<()>> {
    let server_handle = start_test_server(TEST_HOST.to_string(), TEST_PORT);
    // Add a longer delay to ensure the server is fully ready
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    server_handle
}

#[tokio::test]
async fn test_mcp_client_connection() {
    // Start the test server before the test
    let server_handle = setup_test_server().await;

    // Test client connection
    let client = McpClientBuilder::new(TEST_URL.to_string())
        .build_and_initialize("mcp-client".to_string(), "0.1.0".to_string())
        .await;

    // Verify client connection was successful
    assert!(
        client.is_ok(),
        "Failed to connect to MCP server: {:?}",
        client.err()
    );

    // No need to explicitly close the client - it will be dropped at the end of the test

    // Shut down the server after the test
    server_handle.abort();
}

#[tokio::test]
async fn test_mcp_client_tool_list() {
    // Start the test server before the test
    let server_handle = setup_test_server().await;

    // Create and open client
    let client = McpClientBuilder::new(TEST_URL.to_string())
        .build_and_initialize("mcp-client".to_string(), "0.1.0".to_string())
        .await
        .expect("Failed to connect to MCP server");

    // Get tool list
    let tools = client
        .list_tools(None, None)
        .await
        .expect("Failed to list tools");

    // Verify the Add tool is in the list
    assert!(!tools.tools.is_empty(), "Tool list is empty");
    let add_tool = tools.tools.iter().find(|t| t.name == "Add");
    assert!(add_tool.is_some(), "Add tool not found in tool list");
    let add_tool = add_tool.unwrap();
    assert_eq!(add_tool.name, "Add");
    assert_eq!(
        add_tool.description.as_deref().unwrap_or(""),
        "Adds two numbers together."
    );

    // Shut down the server after the test
    server_handle.abort();
}

#[tokio::test]
async fn test_mcp_tool_call() {
    // Start the test server before the test
    let server_handle = setup_test_server().await;

    // Create and open client
    let client = McpClientBuilder::new(TEST_URL.to_string())
        .build_and_initialize("mcp-client".to_string(), "0.1.0".to_string())
        .await
        .expect("Failed to connect to MCP server");

    // Get tool list
    let tools = client
        .list_tools(None, None)
        .await
        .expect("Failed to list tools");
    let add_tool = tools
        .tools
        .iter()
        .find(|t| t.name == "Add")
        .unwrap()
        .clone();

    // Create McpTool from the tool definition
    let mcp_tool = McpTool::from_mcp_server(add_tool, client.clone());

    // Convert to Tool and call it
    let tool: crate::ai::tool::Tool = mcp_tool.into();

    // Test with valid parameters
    let result = tool.call(json!({"a": 5, "b": 3})).await;
    assert!(result.is_ok(), "Tool call failed: {:?}", result.err());
    let value = result.unwrap();

    // The result should be "8" (as a string or number)
    match value {
        Value::String(s) => assert_eq!(s, "8"),
        Value::Number(n) => assert_eq!(n.as_f64().unwrap(), 8.0),
        _ => panic!("Unexpected result type: {:?}", value),
    }

    // Test with invalid parameters (missing parameter)
    let result = tool.call(json!({"a": 5})).await;
    assert!(
        result.is_err(),
        "Tool call with invalid parameters should fail"
    );

    // No need to explicitly close the client - it will be dropped at the end of the test

    // Shut down the server after the test
    server_handle.abort();
}

#[tokio::test]
async fn test_mcp_client_with_headers() {
    // Start the test server before the test
    let server_handle = setup_test_server().await;

    // Test client with custom headers
    let client = McpClientBuilder::new(TEST_URL.to_string())
        .with_header("X-Test-Header", "test-value")
        .build_and_initialize("mcp-client".to_string(), "0.1.0".to_string())
        .await;

    // Verify client connection was successful
    assert!(
        client.is_ok(),
        "Failed to connect to MCP server with custom headers: {:?}",
        client.err()
    );

    // Shut down the server after the test
    server_handle.abort();
}
