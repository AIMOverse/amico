use async_trait::async_trait;

use super::{
    provider::Provider,
    tool::{Tool, ToolSet},
};
use crate::ai::errors::ServiceError;

/// A Service executes a certain AI task, such as generating text.
/// using a series of model provider calls.
///
/// A service should contain a context that is used to configure the service.
#[async_trait]
pub trait Service: Send + Sync {
    /// The LLM API provider type the service uses
    type Provider: Provider;

    /// A service should be built from a context
    fn from(context: ServiceContext<Self::Provider>) -> Self
    where
        Self: Sized;

    /// Gets the context of the service
    fn ctx(&self) -> &ServiceContext<Self::Provider>;

    /// Gets a mutable reference to the context of the service
    fn mut_ctx(&mut self) -> &mut ServiceContext<Self::Provider>;

    /// Generates text based on a prompt.
    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError>;
}

/// The context of a Service.
pub struct ServiceContext<P>
where
    P: Provider,
{
    pub system_prompt: String,
    pub provider: P,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: u64,
    pub tools: ToolSet,
}

impl<P> ServiceContext<P>
where
    P: Provider,
{
    /// Updates the context with a function.
    pub fn update<F>(&mut self, update: F)
    where
        F: Fn(&mut ServiceContext<P>),
    {
        update(self);
    }
}

/// A ServiceBuilder allows to configure a Service before it is used.
pub struct ServiceBuilder<P>
where
    P: Provider,
{
    /// Temporarily stores tools in a vector.
    /// These are moved into the ServiceContext when the builder is built.
    tool_list: Vec<Tool>,
    system_prompt: String,
    provider: P,
    model: String,
    temperature: f64,
    max_tokens: u64,
}

impl<P> ServiceBuilder<P>
where
    P: Provider,
{
    /// Creates a new `ServiceBuilder` with default values.
    pub fn new(provider: P) -> Self {
        Self {
            tool_list: Vec::new(),
            system_prompt: String::new(),
            provider,
            model: String::new(),
            temperature: 0.2, // Default value
            max_tokens: 1000, // Default value
        }
    }

    /// Sets the model for the Service.
    pub fn model(mut self, model: String) -> Self {
        self.model = model;
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
        S: Service<Provider = P>,
    {
        S::from(ServiceContext {
            provider: self.provider,
            model: self.model,
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
    use crate::ai::completion::{CompletionRequest, CompletionRequestBuilder};
    use crate::ai::errors::CompletionError;
    use crate::ai::provider::ModelChoice;

    // Structs for testing

    struct TestProvider;

    struct TestService {
        ctx: ServiceContext<TestProvider>,
    }

    #[async_trait]
    impl Provider for TestProvider {
        async fn completion(
            &self,
            _req: &CompletionRequest,
        ) -> Result<ModelChoice, CompletionError> {
            Ok(ModelChoice::Message("test".to_string()))
        }
    }

    #[async_trait]
    impl Service for TestService {
        type Provider = TestProvider;

        fn from(context: ServiceContext<TestProvider>) -> Self {
            TestService { ctx: context }
        }

        fn ctx(&self) -> &ServiceContext<TestProvider> {
            &self.ctx
        }

        fn mut_ctx(&mut self) -> &mut ServiceContext<TestProvider> {
            &mut self.ctx
        }

        async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
            // Build the request
            let request = CompletionRequestBuilder::from_ctx(&self.ctx)
                .prompt(prompt)
                .build();

            // Perform the completion
            self.ctx
                .provider
                .completion(&request)
                .await
                .map(|choice| match choice {
                    ModelChoice::Message(text) => Ok(text),
                    _ => {
                        return Err(ServiceError::UnexpectedResponse(
                            "Expected a message, got a tool call".to_string(),
                        ))
                    }
                })
                .unwrap()
        }
    }

    /// Builds a test service
    fn build_test_service() -> TestService {
        ServiceBuilder::new(TestProvider)
            .model("test".to_string())
            .system_prompt("test".to_string())
            .temperature(0.2)
            .max_tokens(100)
            .build::<TestService>()
    }

    #[tokio::test]
    async fn test_build_service() {
        let service = build_test_service();

        // Ensure the service is dynamically compatible
        let mut service: Box<dyn Service<Provider = TestProvider>> = Box::new(service);

        assert_eq!(service.ctx().system_prompt, "test".to_string());
        assert_eq!(service.ctx().model, "test".to_string());

        let response = service.generate_text("test".to_string()).await.unwrap();
        assert_eq!(response, "test".to_string());
    }

    #[test]
    fn test_update_context() {
        let service = build_test_service();

        // Ensure the service is dynamically compatible
        let mut service: Box<dyn Service<Provider = TestProvider>> = Box::new(service);

        service.mut_ctx().update(|ctx| {
            ctx.system_prompt = "new test".to_string();
            ctx.model = "new test".to_string();
            ctx.temperature = 0.3;
            ctx.max_tokens = 200;
            ctx.tools = ToolSet::from(vec![]);
        });

        assert_eq!(service.ctx().system_prompt, "new test".to_string());
        assert_eq!(service.ctx().model, "new test".to_string());
        assert_eq!(service.ctx().temperature, 0.3);
        assert_eq!(service.ctx().max_tokens, 200);
        assert_eq!(service.ctx().tools.tools.len(), 0);
    }
}
