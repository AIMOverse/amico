use async_trait::async_trait;

use crate::ai::{
    completion::{Error, Model},
    tool::{Tool, ToolSet},
};

#[cfg(feature = "mcp-client")]
use crate::ai::mcp::{McpClient, McpTool};

/// A completion `Session` is responsible for generating text based on a prompt,
/// managing the context of the session, and calling tools.
///
/// A session should contain a context that is used to configure the session.
pub trait Session {
    /// The completion `Model` type the session uses
    type Model: Model;

    /// A session should define how it is built from a context.
    fn from_ctx(ctx: SessionContext<Self::Model>) -> Self;

    /// Generates text based on a prompt.
    fn generate_text(
        &mut self,
        prompt: String,
    ) -> impl Future<Output = Result<String, Error>> + Send;
}

#[async_trait]
pub trait SessionDyn {
    /// Generates text based on a prompt.
    async fn generate_text_dyn(&mut self, prompt: String) -> Result<String, Error>;
}

#[async_trait(?Send)]
pub trait SessionLocal {
    /// Generates text based on a prompt.
    async fn generate_text_local(&mut self, prompt: String) -> Result<String, Error>;
}

#[async_trait]
impl<T: Session + Send> SessionDyn for T {
    async fn generate_text_dyn(&mut self, prompt: String) -> Result<String, Error> {
        self.generate_text(prompt).await
    }
}

#[async_trait(?Send)]
impl<T: Session> SessionLocal for T {
    async fn generate_text_local(&mut self, prompt: String) -> Result<String, Error> {
        self.generate_text(prompt).await
    }
}

/// The context of a Session.
#[derive(Debug)]
pub struct SessionContext<M>
where
    M: Model,
{
    pub system_prompt: String,
    pub model: M,
    pub model_name: String,
    pub temperature: f64,
    pub max_tokens: u64,
    pub tools: ToolSet,
}

impl<M> SessionContext<M>
where
    M: Model,
{
    /// Updates the context with a function.
    pub fn update<F>(&mut self, update: F)
    where
        F: Fn(&mut SessionContext<M>),
    {
        update(self);
    }
}

/// A SessionBuilder allows to configure a Session before it is used.
pub struct SessionBuilder<M>
where
    M: Model,
{
    /// Temporarily stores tools in a vector.
    ///
    /// These tools will be moved into the `SessionContext` when the builder is built.
    tool_list: Vec<Tool>,
    system_prompt: String,
    completion_model: M,
    model_name: String,
    temperature: f64,
    max_tokens: u64,
}

impl<M> SessionBuilder<M>
where
    M: Model,
{
    /// Creates a new `SessionBuilder` with default values.
    pub fn new(completion_model: M) -> Self {
        Self {
            tool_list: Vec::new(),
            system_prompt: String::new(),
            completion_model,
            model_name: String::new(),
            temperature: 0.2, // Default value
            max_tokens: 1000, // Default value
        }
    }

    /// Sets the model for the Session.
    pub fn model(mut self, model_name: String) -> Self {
        self.model_name = model_name;
        self
    }

    /// Set the system prompt for the Session.
    pub fn system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = prompt;
        self
    }

    /// Add a tool to the Session.
    pub fn tool(mut self, tool: Tool) -> Self {
        self.tool_list.push(tool);
        self
    }

    /// Add a list of tools to the Session.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tool_list.extend(tools);
        self
    }

    /// Add a MCP tool to the Session by definition.
    #[cfg(feature = "mcp-client")]
    pub fn mcp_tool_definition(
        mut self,
        tool_definition: mcp_core::types::Tool,
        mcp_client: McpClient,
    ) -> Self {
        self.tool_list
            .push(McpTool::from_mcp_server(tool_definition, mcp_client).into());
        self
    }

    /// Add all MCP tools from a server to the Session.
    #[cfg(feature = "mcp-client")]
    pub async fn mcp_tools_from_server(mut self, mcp_client: McpClient) -> anyhow::Result<Self> {
        mcp_client
            .list_tools(None, None)
            .await?
            .tools
            .iter()
            .for_each(|tool| {
                self.tool_list
                    .push(McpTool::from_mcp_server(tool.to_owned(), mcp_client.clone()).into());
            });

        Ok(self)
    }

    /// Add a MCP tool to the Session by name.
    #[cfg(feature = "mcp-client")]
    pub async fn mcp_tool(
        mut self,
        tool_name: String,
        mcp_client: McpClient,
    ) -> anyhow::Result<Self> {
        let mcp_client = mcp_client.clone();
        // Find the tool with specified name
        let tools = mcp_client.list_tools(None, None).await?;
        let tool = tools
            .tools
            .iter()
            .find(|tool| tool.name == tool_name)
            .ok_or(anyhow::anyhow!("Tool {} not found", tool_name))?;

        self.tool_list
            .push(McpTool::from_mcp_server(tool.to_owned(), mcp_client.clone()).into());

        Ok(self)
    }

    /// Sets the temperature for the Session.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Sets the max tokens for the Session.
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Build the Session.
    pub fn build<S>(self) -> S
    where
        S: Session<Model = M>,
    {
        S::from_ctx(SessionContext {
            model: self.completion_model,
            model_name: self.model_name,
            system_prompt: self.system_prompt,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tools: ToolSet::from(self.tool_list),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ai::completion::{Error, ModelChoice, Request, RequestBuilder};
    use crate::ai::tool::ToolBuilder;

    // Structs for testing

    struct TestCompletionModel;

    struct TestSession {
        ctx: SessionContext<TestCompletionModel>,
    }

    impl Model for TestCompletionModel {
        async fn completion(&self, _req: &Request) -> Result<ModelChoice, Error> {
            Ok(ModelChoice::Message("test".to_string()))
        }
    }

    impl Session for TestSession {
        type Model = TestCompletionModel;

        fn from_ctx(context: SessionContext<TestCompletionModel>) -> Self {
            TestSession { ctx: context }
        }

        async fn generate_text(&mut self, prompt: String) -> Result<String, Error> {
            // Build the request
            let request = RequestBuilder::from_ctx(&self.ctx).prompt(prompt).build();

            // Perform the completion
            self.ctx
                .model
                .completion(&request)
                .await
                .map(|choice| match choice {
                    ModelChoice::Message(text) => Ok(text),
                    _ => {
                        return Err(Error::BadResponse(
                            "Expected a message, got a tool call".to_string(),
                        ));
                    }
                })
                .unwrap()
        }
    }

    /// Builds a test session
    fn build_test_session() -> TestSession {
        SessionBuilder::new(TestCompletionModel)
            .model("test".to_string())
            .system_prompt("test".to_string())
            .temperature(0.2)
            .max_tokens(100)
            // Test adding tools
            .tool(build_test_tool(1))
            .tool(build_test_tool(2))
            // Test adding a list of tools
            .tools(vec![build_test_tool(3), build_test_tool(4)])
            // Test adding tools after a list of tools are added
            .tool(build_test_tool(5))
            .build::<TestSession>()
    }

    /// Builds a test tool
    fn build_test_tool(id: i32) -> Tool {
        ToolBuilder::new()
            .name(&format!("test_{}", id))
            .description("test")
            .parameters(serde_json::json!({}))
            .build(|_args| Ok(serde_json::json!({"message": "ok"})))
    }

    #[tokio::test]
    async fn test_build_session() {
        let mut session = build_test_session();

        assert_eq!(session.ctx.system_prompt, "test".to_string());
        assert_eq!(session.ctx.model_name, "test".to_string());
        assert_eq!(session.ctx.temperature, 0.2);
        assert_eq!(session.ctx.max_tokens, 100);
        assert_eq!(session.ctx.tools.tools.len(), 5);

        let response = session.generate_text("test".to_string()).await.unwrap();
        assert_eq!(response, "test".to_string());
    }

    #[test]
    fn test_update_context() {
        let mut session = build_test_session();

        session.ctx.update(|ctx| {
            ctx.system_prompt = "new test".to_string();
            ctx.model_name = "new test".to_string();
            ctx.temperature = 0.3;
            ctx.max_tokens = 200;
            ctx.tools = ToolSet::from(vec![]);
        });

        assert_eq!(session.ctx.system_prompt, "new test".to_string());
        assert_eq!(session.ctx.model_name, "new test".to_string());
        assert_eq!(session.ctx.temperature, 0.3);
        assert_eq!(session.ctx.max_tokens, 200);
        assert_eq!(session.ctx.tools.tools.len(), 0);
    }

    #[tokio::test]
    async fn test_session_dyn_compatibility() {
        let session = build_test_session();

        // Ensure the session is dynamically compatible
        let mut boxed: Box<dyn SessionDyn> = Box::new(session);

        let response = boxed.generate_text_dyn("test".to_string()).await.unwrap();
        assert_eq!(response, "test".to_string());
    }
}
