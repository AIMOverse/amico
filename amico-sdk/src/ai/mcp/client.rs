//! Re-exports MCP client components to use from `mcp-core`

use std::collections::HashMap;

use mcp_core::{
    client::{Client, ClientBuilder, SecureValue},
    protocol::RequestOptions,
    transport::{ClientSseTransport, ClientSseTransportBuilder, ClientStdioTransport},
    types::{ClientCapabilities, Implementation},
};

use crate::resource::Resource;

/// MCP feature for Amico uses SSE Transport.
pub type McpSseTransport = ClientSseTransport;

/// The MCP Client using SSE Transport.
pub type McpSseClient = Client<McpSseTransport>;

/// MCP feature for Amico can also use stdio Transport for local commands.
pub type McpCommandTransport = ClientStdioTransport;

/// The MCP Client using command Transport.
pub type McpCommandClient = Client<McpCommandTransport>;

/// The MCP Client using the chosen transport.
#[derive(Clone)]
pub enum McpClient {
    /// SSE Transport based client
    Sse(McpSseClient),
    /// Command Transport based client
    Command(McpCommandClient),
}

/// The resource type representing a MCP Client.
pub type McpResource = Resource<McpClient>;

impl McpResource {
    /// Create a new MCP resource from a client.
    ///
    /// # Arguments
    ///
    /// * `client` - The MCP client.
    ///
    /// # Returns
    ///
    /// A new `McpResource`.
    pub fn from_client(client: McpClient) -> Self {
        Resource::new("MCP Client Resource".to_string(), client)
    }
}

/// The resource builder for MCP Client using SSE Transport.
///
/// This builder combines the transport and client builders.
pub struct McpClientBuilder {
    strict: bool,
    env: Option<HashMap<String, SecureValue>>,
    server_url: String,
    bearer_token: Option<String>,
    headers: HashMap<String, String>,
}

impl McpClientBuilder {
    /// Create a new MCP client resource builder.
    ///
    /// # Arguments
    ///
    /// * `server_url` - The URL of the MCP server.
    ///
    /// # Returns
    ///
    /// A new `McpClientBuilder`.
    pub fn new(server_url: String) -> Self {
        Self {
            strict: false,
            env: None,
            server_url,
            bearer_token: None,
            headers: HashMap::new(),
        }
    }

    /// Add a secure value to the environment.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the secure value.
    /// * `value` - The secure value.
    ///
    /// # Returns
    ///
    /// A new `McpClientResourceBuilder` with the secure value added.
    pub fn with_secure_value(mut self, key: impl Into<String>, value: SecureValue) -> Self {
        // Copied from `mcp-core`'s `ClientBuilder::with_secure_value`
        match &mut self.env {
            Some(env) => {
                env.insert(key.into(), value);
            }
            None => {
                let mut new_env = HashMap::new();
                new_env.insert(key.into(), value);
                self.env = Some(new_env);
            }
        }
        self
    }

    /// Set the client to use strict mode.
    ///
    /// # Returns
    ///
    /// A new `McpClientBuilder` with strict mode enabled.
    pub fn use_strict(mut self) -> Self {
        self.strict = true;
        self
    }

    /// Set the client strict mode value.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `McpClientBuilder` with strict mode value set.
    pub fn with_strict(&mut self) -> &mut Self {
        self.strict = true;
        self
    }

    /// Set the bearer token for the client.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token.
    ///
    /// # Returns
    ///
    /// A new `McpClientBuilder` with the bearer token set.
    pub fn with_bearer_token(mut self, token: String) -> Self {
        self.bearer_token = Some(token);
        self
    }

    /// Set a header for the client.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// A new `McpClientBuilder` with the header set.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Build the client.
    ///
    /// # Returns
    ///
    /// A new `McpClient`.
    pub fn build(self) -> McpClient {
        // Build the transport
        let mut transport_builder = ClientSseTransportBuilder::new(self.server_url);

        if self.bearer_token.is_some() {
            transport_builder = transport_builder.with_bearer_token(self.bearer_token.unwrap());
        }

        if !self.headers.is_empty() {
            transport_builder = self
                .headers
                .into_iter()
                .fold(transport_builder, |builder, (k, v)| {
                    builder.with_header(k, v)
                });
        }

        let transport = transport_builder.build();

        // Build the client
        let mut client_builder = ClientBuilder::new(transport);

        if let Some(env) = self.env {
            if !env.is_empty() {
                client_builder = env.into_iter().fold(client_builder, |builder, (k, v)| {
                    builder.with_secure_value(k, v)
                });
            }
        }

        client_builder = client_builder.with_strict(self.strict);

        McpClient::Sse(client_builder.build())
    }

    /// Build the client, open it and initialize it with default capabilities.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the client implementation.
    /// * `version` - The version of the client implementation.
    ///
    /// # Returns
    ///
    /// A new `McpClient`.
    pub async fn build_and_initialize(
        self,
        name: String,
        version: String,
    ) -> anyhow::Result<McpClient> {
        let client = self.build();

        match &client {
            McpClient::Sse(sse_client) => {
                // Open the client
                sse_client.open().await?;

                // Initialize the client with proper capabilities
                let mut capabilities = ClientCapabilities::default();
                capabilities.experimental = Some(serde_json::json!({}));
                capabilities.sampling = Some(serde_json::json!({}));
                
                // Create roots with a boolean listChanged property
                let roots_obj = serde_json::json!({
                    "listChanged": false
                });
                capabilities.roots = Some(serde_json::from_value(roots_obj).unwrap());

                let _init_res = sse_client
                    .initialize(
                        Implementation { name, version },
                        capabilities,
                    )
                    .await?;
            }
            McpClient::Command(_) => unreachable!("SSE builder can only build SSE clients"),
        }

        Ok(client)
    }
}

/// The resource builder for MCP Client using Command Transport.
///
/// This builder creates a client that communicates with a local command.
pub struct McpCommandClientBuilder {
    strict: bool,
    env: Option<HashMap<String, SecureValue>>,
    program: String,
    args: Vec<String>,
}

impl McpCommandClientBuilder {
    /// Create a new MCP command client resource builder.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute, e.g. "npx".
    /// * `args` - Arguments to pass to the command, e.g. ["@modelcontextprotocol/some-mcp"].
    ///
    /// # Returns
    ///
    /// A new `McpCommandClientBuilder`.
    pub fn new(command: String, args: Vec<String>) -> Self {
        Self {
            strict: false,
            env: None,
            program: command,
            args,
        }
    }

    /// Add a secure value to the environment.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the secure value.
    /// * `value` - The secure value.
    ///
    /// # Returns
    ///
    /// A new `McpCommandClientBuilder` with the secure value added.
    pub fn with_secure_value(mut self, key: impl Into<String>, value: SecureValue) -> Self {
        match &mut self.env {
            Some(env) => {
                env.insert(key.into(), value);
            }
            None => {
                let mut new_env = HashMap::new();
                new_env.insert(key.into(), value);
                self.env = Some(new_env);
            }
        }
        self
    }

    /// Set the client to use strict mode.
    ///
    /// # Returns
    ///
    /// A new `McpCommandClientBuilder` with strict mode enabled.
    pub fn use_strict(mut self) -> Self {
        self.strict = true;
        self
    }

    /// Build the client.
    ///
    /// # Returns
    ///
    /// A new `McpClient` or an error if the transport could not be created.
    pub fn build(self) -> anyhow::Result<McpClient> {
        // Convert string args to &str references for ClientStdioTransport::new
        let args_refs: Vec<&str> = self.args.iter().map(AsRef::as_ref).collect();
        
        // Build the transport
        let transport = ClientStdioTransport::new(&self.program, &args_refs)?;

        // Build the client
        let mut client_builder = ClientBuilder::new(transport);

        if let Some(env) = self.env {
            if !env.is_empty() {
                client_builder = env.into_iter().fold(client_builder, |builder, (k, v)| {
                    builder.with_secure_value(k, v)
                });
            }
        }

        client_builder = client_builder.with_strict(self.strict);

        Ok(McpClient::Command(client_builder.build()))
    }

    /// Build the client, open it and initialize it with default capabilities.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the client implementation.
    /// * `version` - The version of the client implementation.
    ///
    /// # Returns
    ///
    /// A new `McpClient` or an error.
    pub async fn build_and_initialize(
        self,
        name: String,
        version: String,
    ) -> anyhow::Result<McpClient> {
        let client = self.build()?;

        match &client {
            McpClient::Command(command_client) => {
                // Open the client
                command_client.open().await?;

                // Initialize the client with proper capabilities
                let mut capabilities = ClientCapabilities::default();
                capabilities.experimental = Some(serde_json::json!({}));
                capabilities.sampling = Some(serde_json::json!({}));
                
                // Create roots with a boolean listChanged property
                let roots_obj = serde_json::json!({
                    "listChanged": false
                });
                capabilities.roots = Some(serde_json::from_value(roots_obj).unwrap());

                let _init_res = command_client
                    .initialize(
                        Implementation { name, version },
                        capabilities,
                    )
                    .await?;
            }
            McpClient::Sse(_) => unreachable!("Command builder can only build Command clients"),
        }

        Ok(client)
    }
}

// Add extension methods to McpClient to forward to the underlying client
impl McpClient {
    /// Open the client connection
    pub async fn open(&self) -> anyhow::Result<()> {
        match self {
            McpClient::Sse(client) => client.open().await,
            McpClient::Command(client) => client.open().await,
        }
    }

    /// Initialize the client with the given implementation details and capabilities
    pub async fn initialize(
        &self,
        implementation: Implementation,
        capabilities: ClientCapabilities,
    ) -> anyhow::Result<mcp_core::types::InitializeResponse> {
        match self {
            McpClient::Sse(client) => client.initialize(implementation, capabilities).await,
            McpClient::Command(client) => client.initialize(implementation, capabilities).await,
        }
    }

    /// List the tools available from the server
    pub async fn list_tools(
        &self,
        cursor: Option<String>,
        request_options: Option<RequestOptions>,
    ) -> anyhow::Result<mcp_core::types::ToolsListResponse> {
        match self {
            McpClient::Sse(client) => client.list_tools(cursor, request_options).await,
            McpClient::Command(client) => client.list_tools(cursor, request_options).await,
        }
    }

    /// Call a tool with parameters
    pub async fn call_tool(
        &self,
        tool_name: &str,
        params: Option<serde_json::Value>,
    ) -> anyhow::Result<mcp_core::types::CallToolResponse> {
        match self {
            McpClient::Sse(client) => client.call_tool(tool_name, params).await,
            McpClient::Command(client) => client.call_tool(tool_name, params).await,
        }
    }
}
