//! # Amico Models Layer
//!
//! This crate provides model abstractions for the Amico V2 framework.
//! Models are categorized by their capability: language, image generation,
//! video generation, speech, and embeddings.
//!
//! ## Design Principles
//!
//! - **Trait-based abstractions**: All models implement the core `Model` trait
//! - **No dynamic dispatch**: Uses generics and associated types
//! - **Async-native**: All model execution returns `impl Future`
//! - **Provider-agnostic**: Business logic doesn't depend on specific providers
//!
//! ## Example
//!
//! ```rust,ignore
//! use amico_models::{Model, LanguageModel};
//!
//! async fn generate_text<M>(model: &M, prompt: &str) -> String
//! where
//!     M: LanguageModel,
//! {
//!     let input = LanguageInput::new(prompt);
//!     let context = M::Context::default();
//!     let output = model.execute(&context, input).await.unwrap();
//!     output.text
//! }
//! ```

use std::future::Future;

// ============================================================
// Chat message types (multi-turn conversation support)
// ============================================================

/// Role of a participant in a chat conversation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatRole {
    /// System instructions
    System,
    /// User messages
    User,
    /// Assistant (model) responses
    Assistant,
    /// Tool call results
    Tool,
}

/// A single message in a chat conversation
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
        }
    }

    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Tool,
            content: content.into(),
        }
    }
}

/// Input for chat-style multi-turn conversation models
#[derive(Debug, Clone)]
pub struct ChatInput {
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

impl ChatInput {
    pub fn new(messages: Vec<ChatMessage>) -> Self {
        Self {
            messages,
            max_tokens: None,
            temperature: None,
        }
    }
}

/// A single chunk of a streaming model response
#[derive(Debug, Clone)]
pub struct StreamChunk {
    /// The text delta in this chunk
    pub delta: String,
    /// Whether this is the final chunk
    pub done: bool,
}

/// Chat model specialization (multi-turn conversations)
pub trait ChatModel: Model<Input = ChatInput, Output = LanguageOutput> {}

/// Streaming chat model — produces a token stream for real-time output.
///
/// The `TokenStream` associated type must yield `Result<StreamChunk, Self::Error>`.
/// Implementations can use channels, async generators, or any async stream.
pub trait StreamingChatModel: ChatModel {
    /// The async token stream type
    type TokenStream: Send;

    /// Start a streaming chat completion, returning a token stream
    fn stream<'a>(
        &'a self,
        context: &'a Self::Context,
        input: ChatInput,
    ) -> impl Future<Output = Result<Self::TokenStream, Self::Error>> + Send + 'a;
}

// ============================================================
// Core model trait
// ============================================================

/// Core model trait - all AI models implement this
pub trait Model {
    /// Context provided to the model (e.g., configuration, state)
    type Context;
    
    /// Input type for the model
    type Input;
    
    /// Output type produced by the model
    type Output;
    
    /// Error type for model execution
    type Error;

    /// Execute the model with given context and input
    fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + 'a;
}

/// Language model input
#[derive(Debug, Clone)]
pub struct LanguageInput {
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
}

impl LanguageInput {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            system_prompt: None,
            max_tokens: None,
            temperature: None,
        }
    }
}

/// Language model output
#[derive(Debug, Clone)]
pub struct LanguageOutput {
    pub text: String,
    pub finish_reason: FinishReason,
    pub usage: TokenUsage,
}

/// Reason why model generation finished
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
}

/// Token usage information
#[derive(Debug, Clone, Copy)]
pub struct TokenUsage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// Language model specialization
pub trait LanguageModel: Model<Input = LanguageInput, Output = LanguageOutput> {
    /// Create a new instance with a system prompt
    fn with_system_prompt(self, prompt: impl Into<String>) -> WithSystemPrompt<Self>
    where
        Self: Sized,
    {
        WithSystemPrompt {
            inner: self,
            system_prompt: prompt.into(),
        }
    }
}

/// Wrapper that adds a system prompt to a language model
pub struct WithSystemPrompt<M> {
    inner: M,
    system_prompt: String,
}

impl<M> Model for WithSystemPrompt<M>
where
    M: LanguageModel + Sync,
    M::Context: Sync,
{
    type Context = M::Context;
    type Input = LanguageInput;
    type Output = LanguageOutput;
    type Error = M::Error;

    async fn execute<'a>(
        &'a self,
        context: &'a Self::Context,
        mut input: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        input.system_prompt = Some(self.system_prompt.clone());
        self.inner.execute(context, input).await
    }
}

impl<M> LanguageModel for WithSystemPrompt<M>
where
    M: LanguageModel + Sync,
    M::Context: Sync,
{}

/// Image generation prompt
#[derive(Debug, Clone)]
pub struct ImagePrompt {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub steps: Option<u32>,
}

/// Generated image
#[derive(Debug, Clone)]
pub struct Image {
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

/// Image format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
}

/// Image generation model
pub trait ImageGenModel: Model<Input = ImagePrompt, Output = Image> {}

/// Video generation prompt
#[derive(Debug, Clone)]
pub struct VideoPrompt {
    pub prompt: String,
    pub duration_seconds: Option<f32>,
    pub fps: Option<u32>,
}

/// Generated video
#[derive(Debug, Clone)]
pub struct Video {
    pub data: Vec<u8>,
    pub format: VideoFormat,
    pub duration_seconds: f32,
    pub fps: u32,
}

/// Video format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    WebM,
}

/// Video generation model
pub trait VideoGenModel: Model<Input = VideoPrompt, Output = Video> {}

/// Audio/Speech input
#[derive(Debug, Clone)]
pub enum AudioInput {
    /// Text to synthesize into speech
    TextToSpeech { text: String, voice: Option<String> },
    /// Audio to transcribe into text
    SpeechToText { audio: Vec<u8> },
}

/// Audio/Speech output
#[derive(Debug, Clone)]
pub enum AudioOutput {
    /// Synthesized speech audio
    Audio { data: Vec<u8>, format: AudioFormat },
    /// Transcribed text
    Text { text: String },
}

/// Audio format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Ogg,
}

/// Speech/Audio model
pub trait SpeechModel: Model<Input = AudioInput, Output = AudioOutput> {}

/// Embedding input
#[derive(Debug, Clone)]
pub struct EmbeddingInput {
    pub text: String,
}

/// Vector embedding
#[derive(Debug, Clone)]
pub struct Vector {
    pub values: Vec<f32>,
    pub dimensions: usize,
}

/// Embedding model
pub trait EmbeddingModel: Model<Input = EmbeddingInput, Output = Vector> {}

/// Model provider trait - provides access to different model types
pub trait ModelProvider {
    type LanguageModel: LanguageModel;
    type ImageModel: ImageGenModel;
    type VideoModel: VideoGenModel;
    type SpeechModel: SpeechModel;
    type EmbeddingModel: EmbeddingModel;
    
    fn language_model(&self) -> &Self::LanguageModel;
    fn image_model(&self) -> &Self::ImageModel;
    fn video_model(&self) -> &Self::VideoModel;
    fn speech_model(&self) -> &Self::SpeechModel;
    fn embedding_model(&self) -> &Self::EmbeddingModel;
}
