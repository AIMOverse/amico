use async_trait::async_trait;

use crate::ai::errors::ServiceError;
use crate::ai::{
    models::CompletionModel,
    tool::{Tool, ToolSet},
};

#[cfg(feature = "mcp-client")]
use crate::ai::mcp::{McpClient, McpTool};

/// A Service executes a certain AI task, such as generating text.
/// using a series of model provider calls.
///
/// A service should contain a context that is used to configure the service.
pub trait CompletionService {
    /// The LLM API provider type the service uses
    type Model: CompletionModel;

    /// A service should be built from a context
    fn from(context: ServiceContext<Self::Model>) -> Self;

    /// Generates text based on a prompt.
    fn generate_text(
        &mut self,
        prompt: String,
    ) -> impl Future<Output = Result<String, ServiceError>> + Send;
}

#[async_trait]
pub trait CompletionServiceDyn {
    /// Generates text based on a prompt.
    async fn generate_text_dyn(&mut self, prompt: String) -> Result<String, ServiceError>;
}

#[async_trait(?Send)]
pub trait CompletionServiceLocal {
    /// Generates text based on a prompt.
    async fn generate_text_local(&mut self, prompt: String) -> Result<String, ServiceError>;
}

#[async_trait]
impl<T: CompletionService + Send> CompletionServiceDyn for T {
    async fn generate_text_dyn(&mut self, prompt: String) -> Result<String, ServiceError> {
        self.generate_text(prompt).await
    }
}

#[async_trait(?Send)]
impl<T: CompletionService> CompletionServiceLocal for T {
    async fn generate_text_local(&mut self, prompt: String) -> Result<String, ServiceError> {
        self.generate_text(prompt).await
    }
}

/// The context of a Service.
#[derive(Debug)]
pub struct ServiceContext<M>
where
    M: CompletionModel,
{
    pub system_prompt: String,
    pub completion_model: M,
    pub model_name: String,
    pub temperature: f64,
    pub max_tokens: u64,
    pub tools: ToolSet,
}

impl<M> ServiceContext<M>
where
    M: CompletionModel,
{
    /// Updates the context with a function.
    pub fn update<F>(&mut self, update: F)
    where
        F: Fn(&mut ServiceContext<M>),
    {
        update(self);
    }
}

/// A ServiceBuilder allows to configure a Service before it is used.
pub struct ServiceBuilder<M>
where
    M: CompletionModel,
{
    /// Temporarily stores tools in a vector.
    /// These are moved into the ServiceContext when the builder is built.
    tool_list: Vec<Tool>,
    system_prompt: String,
    completion_model: M,
    model_name: String,
    temperature: f64,
    max_tokens: u64,
}

impl<M> ServiceBuilder<M>
where
    M: CompletionModel,
{
    /// Creates a new `ServiceBuilder` with default values.
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

    /// Sets the model for the Service.
    pub fn model(mut self, model_name: String) -> Self {
        self.model_name = model_name;
        self
    }

    /// Set the system prompt for the Service.
    pub fn system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = prompt;
        self
    }

    /// Add a tool to the Service.
    pub fn tool(mut self, tool: Tool) -> Self {
        self.tool_list.push(tool);
        self
    }

    /// Add a list of tools to the Service.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tool_list.extend(tools);
        self
    }

    /// Add a MCP tool to the Service by definition.
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

    /// Add all MCP tools from a server to the Service.
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

    /// Add a MCP tool to the Service by name.
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

    /// Sets the temperature for the Service.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    /// Sets the max tokens for the Service.
    pub fn max_tokens(mut self, max_tokens: u64) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Build the Service.
    pub fn build<S>(self) -> S
    where
        S: CompletionService<Model = M>,
    {
        S::from(ServiceContext {
            completion_model: self.completion_model,
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
    use crate::ai::errors::CompletionModelError;
    use crate::ai::models::ModelChoice;
    use crate::ai::models::{CompletionRequest, CompletionRequestBuilder};
    use crate::ai::tool::ToolBuilder;

    // Structs for testing

    struct TestCompletionModel;

    struct TestService {
        ctx: ServiceContext<TestCompletionModel>,
    }

    impl CompletionModel for TestCompletionModel {
        async fn completion(
            &self,
            _req: &CompletionRequest,
        ) -> Result<ModelChoice, CompletionModelError> {
            Ok(ModelChoice::Message("test".to_string()))
        }
    }

    impl CompletionService for TestService {
        type Model = TestCompletionModel;

        fn from(context: ServiceContext<TestCompletionModel>) -> Self {
            TestService { ctx: context }
        }

        async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
            // Build the request
            let request = CompletionRequestBuilder::from_ctx(&self.ctx)
                .prompt(prompt)
                .build();

            // Perform the completion
            self.ctx
                .completion_model
                .completion(&request)
                .await
                .map(|choice| match choice {
                    ModelChoice::Message(text) => Ok(text),
                    _ => {
                        return Err(ServiceError::UnexpectedResponse(
                            "Expected a message, got a tool call".to_string(),
                        ));
                    }
                })
                .unwrap()
        }
    }

    /// Builds a test service
    fn build_test_service() -> TestService {
        ServiceBuilder::new(TestCompletionModel)
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
            .build::<TestService>()
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
    async fn test_build_service() {
        let mut service = build_test_service();

        assert_eq!(service.ctx.system_prompt, "test".to_string());
        assert_eq!(service.ctx.model_name, "test".to_string());
        assert_eq!(service.ctx.temperature, 0.2);
        assert_eq!(service.ctx.max_tokens, 100);
        assert_eq!(service.ctx.tools.tools.len(), 5);

        let response = service.generate_text("test".to_string()).await.unwrap();
        assert_eq!(response, "test".to_string());
    }

    #[test]
    fn test_update_context() {
        let mut service = build_test_service();

        service.ctx.update(|ctx| {
            ctx.system_prompt = "new test".to_string();
            ctx.model_name = "new test".to_string();
            ctx.temperature = 0.3;
            ctx.max_tokens = 200;
            ctx.tools = ToolSet::from(vec![]);
        });

        assert_eq!(service.ctx.system_prompt, "new test".to_string());
        assert_eq!(service.ctx.model_name, "new test".to_string());
        assert_eq!(service.ctx.temperature, 0.3);
        assert_eq!(service.ctx.max_tokens, 200);
        assert_eq!(service.ctx.tools.tools.len(), 0);
    }

    #[test]
    fn test_service_dyn_compatibility() {
        let service = build_test_service();

        // Ensure the service is dynamically compatible
        let _: Box<dyn CompletionServiceDyn> = Box::new(service);
    }
}
