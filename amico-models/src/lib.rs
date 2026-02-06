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

/// A single part of a multimodal message.
///
/// Modern chat models accept messages composed of multiple content parts,
/// e.g. a text prompt accompanied by an image. Each variant carries the
/// data for one modality.
#[derive(Debug, Clone, PartialEq)]
pub enum ContentPart {
    /// Plain text content
    Text { text: String },
    /// An image, either inline (base64) or referenced by URL
    Image { source: ImageSource },
    /// Inline audio data
    Audio { data: Vec<u8>, format: AudioFormat },
}

impl ContentPart {
    /// Convenience: create a text content part.
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text { text: text.into() }
    }

    /// Convenience: create an image content part from a URL.
    pub fn image_url(url: impl Into<String>) -> Self {
        Self::Image {
            source: ImageSource::Url {
                url: url.into(),
            },
        }
    }

    /// Convenience: create an image content part from raw bytes.
    pub fn image_base64(data: Vec<u8>, media_type: impl Into<String>) -> Self {
        Self::Image {
            source: ImageSource::Base64 {
                data,
                media_type: media_type.into(),
            },
        }
    }

    /// Convenience: create an audio content part.
    pub fn audio(data: Vec<u8>, format: AudioFormat) -> Self {
        Self::Audio { data, format }
    }
}

/// Source of an image in a content part.
#[derive(Debug, Clone, PartialEq)]
pub enum ImageSource {
    /// Image hosted at a URL
    Url { url: String },
    /// Base64-encoded inline image
    Base64 { data: Vec<u8>, media_type: String },
}

/// A single message in a chat conversation.
///
/// Messages can contain one or more [`ContentPart`]s, supporting both
/// simple text-only messages and rich multimodal content (images, audio).
///
/// For the common text-only case, convenience constructors like
/// [`ChatMessage::user`] accept a plain string.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: Vec<ContentPart>,
}

impl ChatMessage {
    /// Create a system message (text only).
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: vec![ContentPart::text(content)],
        }
    }

    /// Create a user message (text only).
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: vec![ContentPart::text(content)],
        }
    }

    /// Create an assistant message (text only).
    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: vec![ContentPart::text(content)],
        }
    }

    /// Create a tool result message (text only).
    pub fn tool(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Tool,
            content: vec![ContentPart::text(content)],
        }
    }

    /// Create a message with explicit multimodal content parts.
    pub fn new(role: ChatRole, content: Vec<ContentPart>) -> Self {
        Self { role, content }
    }

    /// Extract the concatenated text from all `Text` parts.
    ///
    /// Returns an empty string when the message contains no text parts.
    pub fn text(&self) -> String {
        self.content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_only_constructors_produce_single_text_part() {
        let msg = ChatMessage::user("hello");
        assert_eq!(msg.role, ChatRole::User);
        assert_eq!(msg.content.len(), 1);
        assert_eq!(msg.content[0], ContentPart::Text { text: "hello".into() });
        assert_eq!(msg.text(), "hello");
    }

    #[test]
    fn all_role_constructors() {
        assert_eq!(ChatMessage::system("s").role, ChatRole::System);
        assert_eq!(ChatMessage::user("u").role, ChatRole::User);
        assert_eq!(ChatMessage::assistant("a").role, ChatRole::Assistant);
        assert_eq!(ChatMessage::tool("t").role, ChatRole::Tool);
    }

    #[test]
    fn multimodal_message_with_text_and_image() {
        let msg = ChatMessage::new(
            ChatRole::User,
            vec![
                ContentPart::text("Describe this image:"),
                ContentPart::image_url("https://example.com/img.png"),
            ],
        );
        assert_eq!(msg.content.len(), 2);
        assert_eq!(msg.text(), "Describe this image:");
    }

    #[test]
    fn text_concatenates_multiple_text_parts() {
        let msg = ChatMessage::new(
            ChatRole::User,
            vec![
                ContentPart::text("Hello "),
                ContentPart::image_url("https://x.com/i.png"),
                ContentPart::text("world"),
            ],
        );
        assert_eq!(msg.text(), "Hello world");
    }

    #[test]
    fn text_returns_empty_for_non_text_content() {
        let msg = ChatMessage::new(
            ChatRole::User,
            vec![ContentPart::image_url("https://example.com/img.png")],
        );
        assert_eq!(msg.text(), "");
    }

    #[test]
    fn content_part_image_base64() {
        let part = ContentPart::image_base64(vec![1, 2, 3], "image/png");
        match &part {
            ContentPart::Image { source: ImageSource::Base64 { data, media_type } } => {
                assert_eq!(data, &[1, 2, 3]);
                assert_eq!(media_type, "image/png");
            }
            _ => panic!("Expected Image::Base64"),
        }
    }

    #[test]
    fn content_part_audio() {
        let part = ContentPart::audio(vec![4, 5], AudioFormat::Mp3);
        match &part {
            ContentPart::Audio { data, format } => {
                assert_eq!(data, &[4, 5]);
                assert_eq!(*format, AudioFormat::Mp3);
            }
            _ => panic!("Expected Audio"),
        }
    }

    #[test]
    fn chat_input_new_works_with_multimodal_messages() {
        let input = ChatInput::new(vec![
            ChatMessage::system("You are helpful."),
            ChatMessage::new(
                ChatRole::User,
                vec![
                    ContentPart::text("What's in this image?"),
                    ContentPart::image_url("https://example.com/img.png"),
                ],
            ),
        ]);
        assert_eq!(input.messages.len(), 2);
    }
}
