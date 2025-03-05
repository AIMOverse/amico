use amico::ai::{
    completion::CompletionRequest,
    errors::{CompletionError, CreationError},
    message::Message,
    provider::{ModelChoice, Provider},
    tool::ToolDefinition,
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use rig::{
    completion::{CompletionModel, CompletionRequest as RigCompletionRequest},
    providers::openai,
};

use crate::interface::{Plugin, PluginCategory, PluginInfo};

lazy_static! {
    /// List of available OpenAI models
    pub static ref OPENAI_MODELS: Vec<&'static str> = vec![
        openai::GPT_4,
        openai::GPT_4O,
        openai::GPT_4O_MINI,
        openai::GPT_4_TURBO,
        openai::GPT_35_TURBO,
    ];
}

// Implement type convertions

/// Convert `sdk`'s `Message` into `rig`'s `Message`
fn into_rig_message(message: &Message) -> rig::completion::Message {
    rig::completion::Message {
        role: message.role.clone(),
        content: message.content(),
    }
}

/// Convert `rig`'s `ModelChoice` into `sdk`'s `ModelChoice`
fn from_rig_choice(choice: rig::completion::ModelChoice) -> ModelChoice {
    match choice {
        rig::completion::ModelChoice::ToolCall(name, id, params) => {
            ModelChoice::ToolCall(name, id, params)
        }
        rig::completion::ModelChoice::Message(msg) => ModelChoice::Message(msg),
    }
}

/// Convert `sdk`'s `Tool` into `rig`'s `ToolDefinition`
fn into_rig_tool_def(def: &ToolDefinition) -> rig::completion::ToolDefinition {
    rig::completion::ToolDefinition {
        name: def.name.clone(),
        description: def.description.clone(),
        parameters: def.parameters.clone(),
    }
}

/// OpenAI provider using `rig-core`
pub struct OpenAI(openai::Client);

impl OpenAI {
    pub fn new(base_url: &str, api_key: &str) -> Result<Self, CreationError>
    where
        Self: Sized,
    {
        Ok(OpenAI(openai::Client::from_url(api_key, base_url)))
    }

    pub fn model_available(&self, model: &str) -> bool {
        // Check if the model name is available
        OPENAI_MODELS.contains(&model)
    }
}

#[async_trait]
impl Provider for OpenAI {
    #[doc = " Completes a prompt with the provider."]
    async fn completion(&self, req: &CompletionRequest) -> Result<ModelChoice, CompletionError> {
        let Self(client) = self;

        if !self.model_available(&req.model) {
            return Err(CompletionError::ModelUnavailable(req.model.clone()));
        }

        let model = client.completion_model(&req.model);

        // Build the rig completion request
        let request = RigCompletionRequest {
            chat_history: req.chat_history.iter().map(into_rig_message).collect(),
            prompt: req.prompt.to_string(),
            preamble: req.system_prompt.clone(),
            temperature: req.temperature,
            max_tokens: req.max_tokens,
            additional_params: None,
            tools: req.tools.iter().map(into_rig_tool_def).collect(),
            documents: Vec::new(),
        };

        // Perform request to the AI model API
        let response = model.completion(request).await;
        // tracing::debug!("OpenAI response: {:?}", response);

        // Convert the rig completion response to a ModelChoice
        match response {
            Ok(res) => Ok(from_rig_choice(res.choice)),
            Err(err) => {
                tracing::error!("API error: {}", err);
                Err(CompletionError::ApiError)
            }
        }
    }
}

impl Plugin for OpenAI {
    fn info(&self) -> &'static PluginInfo {
        &PluginInfo {
            name: "StdOpenAIProvider",
            category: PluginCategory::Service,
        }
    }
}
