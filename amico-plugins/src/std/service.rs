use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::completion::CompletionRequestBuilder;
use amico::ai::{
    errors::{CompletionError, ServiceError, ToolCallError},
    message::Message,
    provider::{ModelChoice, Provider},
    service::ServiceContext,
};
use async_trait::async_trait;

fn debug_history(history: &Vec<Message>) -> String {
    let mut messages = String::new();

    // Convert message to a prettier shorter string
    for m in history.iter() {
        match m {
            Message::Assistant(text) => messages.push_str(&format!("a: {}\n", text)),
            Message::User(text) => messages.push_str(&format!("u: {}\n", text)),
            Message::ToolCall(name, id, params) => {
                messages.push_str(&format!("tc: {}[{}] ({})\n", name, id, params))
            }
            Message::ToolResult(name, id, params) => {
                messages.push_str(&format!("tr: {}[{}] => {}\n", name, id, params))
            }
        }
    }

    messages
}

pub struct InMemoryService<P>
where
    P: Provider,
{
    /// The context config for the service
    pub ctx: ServiceContext<P>,

    /// In-memory Chat history storage
    pub history: Vec<Message>,
}

impl<P: Provider> Plugin for InMemoryService<P> {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdInMemoryService",
            category: PluginCategory::Service,
        }
    }
}

#[async_trait]
impl<P> amico::ai::service::Service<P> for InMemoryService<P>
where
    P: Provider,
{
    fn from(context: ServiceContext<P>) -> Self
    where
        Self: Sized,
    {
        Self {
            ctx: context,
            history: Vec::new(),
        }
    }

    fn ctx(&self) -> &ServiceContext<P> {
        &self.ctx
    }

    fn mut_ctx(&mut self) -> &mut ServiceContext<P> {
        &mut self.ctx
    }

    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
        tracing::debug!(
            "Requesting completion with history:\n{}",
            debug_history(&self.history)
        );
        let request = CompletionRequestBuilder::from_ctx(&self.ctx)
            .prompt(prompt.clone())
            .history(self.history.clone())
            .build();

        let response = self.ctx.provider.completion(&request).await;

        match response {
            Ok(choice) => match choice {
                ModelChoice::Message(msg) => {
                    tracing::debug!("Received message response: {}", msg);

                    // Add the new response to the history list
                    self.history.push(Message::user(prompt.clone()));
                    self.history.push(Message::assistant(msg.clone()));

                    tracing::debug!("Updated history: \n{}", debug_history(&self.history));

                    // Return the response message
                    Ok(msg)
                }
                ModelChoice::ToolCall(name, id, params) => {
                    tracing::debug!("Calling {} ({}) with params {}", name, id, params);

                    // Execute the tool
                    if let Some(tool) = self.ctx.tools.get(&name) {
                        match tool.call(params.clone()).await {
                            Ok(res) => {
                                // Successfully called the tool
                                tracing::debug!("Tool call succeeded with result: {}", res);

                                self.history.push(Message::user(prompt.clone()));
                                self.history.push(Message::tool_call(
                                    name.clone(),
                                    id.clone(),
                                    params.clone(),
                                ));
                                self.history.push(Message::tool_result(
                                    name.clone(),
                                    id.clone(),
                                    res.clone(),
                                ));

                                tracing::debug!(
                                    "Updated history: \n{}",
                                    debug_history(&self.history)
                                );

                                tracing::debug!("Re-generating text");
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
                            }
                            Err(err) => {
                                // Failed to call the tool
                                tracing::debug!("Tool call failed with error: {}", err);
                                tracing::error!("Failed to call tool: {}", err.to_string());

                                self.history.push(Message::user(prompt.clone()));
                                self.history.push(Message::tool_call(
                                    name.clone(),
                                    id.clone(),
                                    params.clone(),
                                ));
                                self.history.push(Message::tool_result(
                                    name.clone(),
                                    id.clone(),
                                    serde_json::json!({
                                        "result": "error",
                                        "message": err.to_string(),
                                    }),
                                ));

                                tracing::debug!(
                                    "Updated history: \n{}",
                                    debug_history(&self.history)
                                );

                                tracing::debug!("Re-generating text");
                                // Re-generate the text with the prompt and the new information
                                self.generate_text(prompt).await
                            }
                        }
                    } else {
                        Err(ServiceError::ToolError(ToolCallError::ToolUnavailable(
                            name,
                        )))
                    }
                }
            },
            Err(err) => {
                tracing::error!("Provider error: {}", err);
                Err(ServiceError::ProviderError(CompletionError::ApiError))
            }
        }
    }
}
