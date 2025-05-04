use amico::ai::{
    errors::CompletionModelError,
    message::Message,
    models::{CompletionRequest, ModelChoice},
    tool::ToolDefinition,
};
use rig::{completion as rc, message as rm, OneOrMany};

/// Convert `sdk`'s `Message` into `rig`'s `Message`
pub fn into_rig_message(message: &Message) -> rc::Message {
    match message {
        Message::Assistant(content) => rc::Message::Assistant {
            content: OneOrMany::one(rm::AssistantContent::text(content.clone())),
        },
        Message::User(content) => rc::Message::User {
            content: OneOrMany::one(rm::UserContent::text(content.clone())),
        },
        Message::ToolCall(name, id, params) => rc::Message::Assistant {
            content: OneOrMany::one(rm::AssistantContent::ToolCall(rm::ToolCall {
                id: id.clone(),
                function: rm::ToolFunction {
                    name: name.clone(),
                    arguments: params.clone(),
                },
            })),
        },
        Message::ToolResult(_, id, result) => rc::Message::User {
            content: OneOrMany::one(rm::UserContent::ToolResult(rm::ToolResult {
                id: id.clone(),
                content: OneOrMany::one(rm::ToolResultContent::text(result.to_string())),
            })),
        },
    }
}

/// Convert `rig`'s `CompletionResponse` into `amico`'s `ModelChoice`
pub fn into_amico_choice<T>(response: rc::CompletionResponse<T>) -> ModelChoice {
    match response.choice.first() {
        rm::AssistantContent::ToolCall(tool_call) => ModelChoice::ToolCall(
            tool_call.function.name,
            tool_call.id,
            tool_call.function.arguments,
        ),
        rm::AssistantContent::Text(text) => ModelChoice::Message(text.text.clone()),
    }
}

/// Convert `rig`'s `CompletionError` into `amico`'s `CompletionModelError`
pub fn into_amico_err(error: rc::CompletionError) -> CompletionModelError {
    CompletionModelError::ProviderError(error.to_string())
}

/// Convert `amico`'s `Tool` into `rig`'s `ToolDefinition`
pub fn into_rig_tool_def(def: &ToolDefinition) -> rig::completion::ToolDefinition {
    rig::completion::ToolDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        parameters: def.parameters.clone(),
    }
}

/// Convert `amico`'s `CompletionRequest` into `rig`'s
pub fn into_rig_request(request: &CompletionRequest) -> rc::CompletionRequest {
    // Documented in `rig-core`:
    // The very last message will always be the prompt (hense why there is *always* one)

    /// Convert chat history vec + prompt into `OneOrMany<Message>`
    fn convert_messages(list: Vec<rm::Message>, prompt: String) -> OneOrMany<rm::Message> {
        let prompt_message = rm::Message::User {
            content: OneOrMany::one(rm::UserContent::text(prompt)),
        };
        if let Ok(mut result) = OneOrMany::many(list) {
            result.push(prompt_message);
            result
        } else {
            // List is empty.
            OneOrMany::one(prompt_message)
        }
    }

    rc::CompletionRequest {
        chat_history: convert_messages(
            request.chat_history.iter().map(into_rig_message).collect(),
            request.prompt.to_string(),
        ),
        preamble: request.system_prompt.clone(),
        temperature: request.temperature,
        max_tokens: request.max_tokens,
        additional_params: None,
        tools: request.tools.iter().map(into_rig_tool_def).collect(),
        documents: Vec::new(),
    }
}
