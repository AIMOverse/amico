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

    /// Generates text based on a prompt.
    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError>;
}

/// The context of a Service.
#[derive(Default)]
pub struct ServiceContext<P>
where
    P: Provider,
{
    pub system_prompt: String,
    pub provider: P,
    pub model: String,
    pub tools: ToolSet,
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
    pub fn new() -> Self {
        Self {
            tool_list: Vec::new(),
            context: ServiceContext::default(),
        }
    }

    /// Set the system prompt for the Service.
    pub fn system_prompt(mut self, prompt: String) -> Self {
        self.context.system_prompt = prompt;
        self
    }

    /// Set the provider for the Service.
    pub fn provider(mut self, provider: P) -> Self {
        self.context.provider = provider;
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

    #[derive(Default)]
    struct TestProvider;

    #[derive(Default)]
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

        async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
            // Build the request
            let request = CompletionRequestBuilder::new()
                .prompt(prompt)
                .system_prompt(self.ctx.system_prompt.clone())
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

    #[test]
    fn test_build_service() {
        let service = ServiceBuilder::new()
            .system_prompt("test".to_string())
            .provider(TestProvider::default())
            .build::<TestService>();

        assert_eq!(service.ctx.system_prompt, "test".to_string());
    }
}
