//! Re-exports MCP client components to use from `mcp-core`

use std::collections::HashMap;

use mcp_core::{
    client::{Client, ClientBuilder, SecureValue},
    transport::{ClientSseTransport, ClientSseTransportBuilder},
    types::{ClientCapabilities, Implementation},
};

use crate::resource::Resource;

/// MCP feature for Amico uses SSE Transport.
pub type McpTransport = ClientSseTransport;

/// The MCP Client using SSE Transport.
pub type McpClient = Client<McpTransport>;

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

/// The resource builder for MCP Client.
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

        client_builder.build()
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

        // Open the client
        client.open().await?;

        // Initialize the client
        let _init_res = client
            .initialize(
                Implementation { name, version },
                ClientCapabilities::default(),
            )
            .await?;

        Ok(client)
    }
}
