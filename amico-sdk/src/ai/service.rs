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
pub trait Service<P>: Send + Sync
where
    P: Provider,
{
    /// A service should be built from a context
    fn from(context: ServiceContext<P>) -> Self
    where
        Self: Sized;

    /// Gets the context of the service
    fn ctx(&self) -> &ServiceContext<P>;

    /// Gets a mutable reference to the context of the service
    fn mut_ctx(&mut self) -> &mut ServiceContext<P>;

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
    pub tools: ToolSet,
}

impl<P> ServiceContext<P>
where
    P: Provider,
{
    /// Updates the system prompt
    pub fn update_system_prompt(&mut self, prompt: String) {
        self.system_prompt = prompt;
    }

    /// Updates the model name
    pub fn update_model(&mut self, model: String) {
        self.model = model;
    }
}

/// A ServiceBuilder allows to configure a Service before it is used.
pub struct ServiceBuilder<P>
where
    P: Provider,
{
    tool_list: Vec<Tool>,
    context: ServiceContext<P>,
}

impl<P> ServiceBuilder<P>
where
    P: Provider,
{
    /// Creates a new `ServiceBuilder` with default values.
    pub fn new(provider: P) -> Self {
        Self {
            tool_list: Vec::new(),
            context: ServiceContext {
                system_prompt: String::new(),
                provider,
                model: String::new(),
                tools: ToolSet::new(),
            },
        }
    }

    /// Sets the model for the Service.
    pub fn model(mut self, model: String) -> Self {
        self.context.model = model;
        self
    }

    /// Set the system prompt for the Service.
    pub fn system_prompt(mut self, prompt: String) -> Self {
        self.context.system_prompt = prompt;
        self
    }

    /// Add a tool to the Service.
    pub fn tool(mut self, tool: Tool) -> Self {
        self.tool_list.push(tool);
        self
    }

    /// Build the Service.
    pub fn build<S>(self) -> S
    where
        S: Service<P>,
    {
        S::from(self.context)
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
    impl Service<TestProvider> for TestService {
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
            let request = CompletionRequestBuilder::new()
                .prompt(prompt)
                .system_prompt(self.ctx.system_prompt.clone())
                .model(self.ctx.model.clone())
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

    #[tokio::test]
    async fn test_build_service() {
        let mut service = ServiceBuilder::new(TestProvider)
            .model("test".to_string())
            .system_prompt("test".to_string())
            .build::<TestService>();

        assert_eq!(service.ctx.system_prompt, "test".to_string());
        assert_eq!(service.ctx.model, "test".to_string());

        let response = service.generate_text("test".to_string()).await.unwrap();
        assert_eq!(response, "test".to_string());
    }
}
