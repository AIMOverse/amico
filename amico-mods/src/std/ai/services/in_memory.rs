use crate::interface::{Plugin, PluginCategory, PluginInfo};
use amico::ai::{
    errors::ServiceError,
    message::Message,
    models::CompletionRequestBuilder,
    models::{CompletionModel, ModelChoice},
    services::ServiceContext,
};

/// Convert a message history to a human-readable brief list for debugging
fn debug_history(history: &[Message]) -> String {
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

#[derive(Debug)]
pub struct InMemoryService<M: CompletionModel + Send> {
    /// The context config for the service
    pub ctx: ServiceContext<M>,

    /// In-memory Chat history storage
    pub history: Vec<Message>,
}

impl<M: CompletionModel + Send> Plugin for InMemoryService<M> {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdInMemoryService",
            category: PluginCategory::Service,
        }
    }
}

impl<M: CompletionModel + Send> amico::ai::services::CompletionService for InMemoryService<M> {
    type Model = M;

    fn from(context: ServiceContext<M>) -> Self {
        Self {
            ctx: context,
            history: Vec::new(),
        }
    }

    async fn generate_text(&mut self, prompt: String) -> Result<String, ServiceError> {
        // Append the new user prompt to chat history.
        self.history.push(Message::User(prompt));

        // Generate the final text
        loop {
            // Call the LLM API wrapper with the current prompt and chat history.
            let request = CompletionRequestBuilder::from_ctx(&self.ctx)
                // We've already added the user prompt to the history, so no need to add it again
                // .prompt(prompt.clone())
                .history(self.history.clone())
                .build();

            // Call the LLM API wrapper with the current prompt and chat history.
            match self.ctx.completion_model.completion(&request).await {
                // When a plain message is received, update the chat history and return the response.
                Ok(ModelChoice::Message(msg)) => {
                    tracing::debug!("Received message response: {}", msg);

                    // Add the new response to the history list
                    self.history.push(Message::assistant(msg.clone()));
                    tracing::debug!("Updated history: \n{}", debug_history(&self.history));

                    // Return the response message
                    return Ok(msg);
                }
                // When a tool call is received, add the tool call to the history, execute it,
                // and append the tool's result to the history before re-asking the LLM.
                Ok(ModelChoice::ToolCall(name, id, params)) => {
                    tracing::debug!("Calling {} ({}) with params {}", name, id, params);

                    // Add the tool call request to chat history
                    self.history
                        .push(Message::tool_call(name.clone(), id.clone(), params.clone()));

                    // Find and execute the tool.
                    let result = if let Some(tool) = self.ctx.tools.get(&name) {
                        // Tool found in the tool set. Execute the tool.
                        tool.call(params.clone())
                            .await
                            .map(|res| {
                                // Successfully called the tool
                                tracing::debug!("Tool call success: {:?}", res);
                                res
                            })
                            .unwrap_or_else(|err| {
                                // Failed to call the tool, but convert the error into result object
                                tracing::warn!("Error during tool call: {}", err);
                                serde_json::json!({
                                    "result": "error",
                                    "message": err.to_string(),
                                })
                            })
                    } else {
                        // Tool not found.
                        tracing::warn!("Failed to find tool");
                        serde_json::json!({
                            "result": "error",
                            "message": format!("Tool {} not found.", name),
                        })
                    };

                    // Update chat history with tool result
                    self.history
                        .push(Message::tool_result(name.clone(), id.clone(), result));
                    tracing::debug!("Updated history: \n{}", debug_history(&self.history));
                }
                // Handle potential errors from the API call.
                Err(err) => {
                    tracing::error!("Provider error: {}", err);
                    return Err(ServiceError::CompletionModelError(err));
                }
            }
        }
    }
}
