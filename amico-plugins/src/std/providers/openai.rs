use amico::ai::{
    chat::{ChatHistory, Message},
    provider::{
        errors::{CompletionError, CreationError},
        ModelChoice, Provider,
    },
};
use async_trait::async_trait;
use lazy_static::lazy_static;
use rig::{
    completion::{CompletionModel, CompletionRequest},
    providers::openai,
};

lazy_static! {
    /// List of available OpenAI models
    static ref OPENAI_MODELS: Vec<&'static str> = vec![
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
        content: message.content.clone(),
    }
}

/// Convert `rig`'s `ModelChoice` into `sdk`'s `ModelChoice`
fn from_rig_choice(choice: rig::completion::ModelChoice) -> ModelChoice {
    match choice {
        rig::completion::ModelChoice::ToolCall(name, _, params) => {
            ModelChoice::ToolCall(name, params)
        }
        rig::completion::ModelChoice::Message(msg) => ModelChoice::Message(msg),
    }
}

/// OpenAI provider using `rig-core`
pub struct OpenAI(openai::Client);

#[async_trait]
impl Provider for OpenAI {
    #[doc = " Creates a new provider."]
    fn new(base_url: Option<&str>, api_key: Option<&str>) -> Result<Self, CreationError>
    where
        Self: Sized,
    {
        if let (Some(api_key), Some(base_url)) = (api_key, base_url) {
            // If both base_url and api_key are provided, use them.
            Ok(OpenAI(openai::Client::from_url(api_key, base_url)))
        } else if let (Some(api_key), None) = (api_key, base_url) {
            // If only api_key is provided, use it.
            Ok(OpenAI(openai::Client::new(api_key)))
        } else {
            // If neither base_url nor api_key is provided,
            // or if only base_url is provided, return an error.
            Err(CreationError::InvalidParam)
        }
    }

    #[doc = " Completes a prompt with the provider."]
    async fn completion(
        &self,
        model: String,
        prompt: String,
        chat_history: &ChatHistory,
    ) -> Result<ModelChoice, CompletionError> {
        let Self(client) = self;

        if !self.model_available(&model).await {
            // TODO: wait for sdk to publish
            // return Err(CompletionError::ModelUnavailable(model));
        }

        let model = client.completion_model(model.as_str());

        // Build the rig completion request
        let request = CompletionRequest {
            chat_history: chat_history.iter().map(|m| into_rig_message(m)).collect(),
            prompt,
            preamble: None,
            temperature: None,
            max_tokens: None,
            additional_params: None,
            tools: Vec::new(),
            documents: Vec::new(),
        };

        // Perform request to the AI model API
        let response = model.completion(request).await;

        // Convert the rig completion response to a ModelChoice
        match response {
            Ok(res) => Ok(from_rig_choice(res.choice)),
            Err(_) => Err(CompletionError::ApiError),
        }
    }

    #[doc = " Checks if a model name is available."]
    async fn model_available(&self, model: &str) -> bool {
        // Check if the model name is available
        OPENAI_MODELS.contains(&model)
    }
}
