use reqwest::multipart;
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use tts::*;

#[derive(Debug, thiserror::Error)]
pub enum TtsError {
    #[error("Failed to read audio file")]
    ReadError(#[from] std::io::Error),
    
    #[error("Failed to initialize TTS engine")]
    TtsInitError(String),
    
    #[error("Failed to synthesize speech")]
    TtsSynthesisError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("Failed to get API key")]
    EnvVarError(#[from] std::env::VarError),

    #[error("Failed to read audio file")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to create multipart request")]
    MultipartError(#[from] reqwest::Error),

    #[error("Failed to set mime type")]
    MimeError(#[from] reqwest::header::InvalidHeaderValue),
}

pub async fn text_to_speech(text: &str, file_path: &str) -> Result<(), TtsError> {
    tracing::debug!("tts text: {}, file: {}", text, file_path);
    
    // Use tokio spawn_blocking for CPU-intensive TTS operations
    tokio::task::spawn_blocking(move || {
        // Initialize TTS with default settings
        let mut tts = match tts::Tts::default() {
            Ok(tts) => tts,
            Err(err) => return Err(TtsError::TtsInitError(err.to_string())),
        };
        
        // Configure for Chinese voice if available
        // Note: You'll need to download Chinese models for Coqui TTS
        // This is a placeholder - actual voice IDs depend on your installed models
        if let Err(err) = tts.set_voice("zh-CN") {
            tracing::warn!("Could not set Chinese voice: {}", err);
            // Fallback to default voice
        }
        
        // Set output file format to WAV
        if let Err(err) = tts.set_output_format(OutputFormat::Wav) {
            return Err(TtsError::TtsInitError(err.to_string()));
        }
        
        // Synthesize speech to file
        match tts.synthesize_to_file(text, file_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(TtsError::TtsSynthesisError(err.to_string())),
        }
    })
    .await
    .unwrap_or_else(|e| Err(TtsError::TtsInitError(e.to_string())))
}

pub async fn speech_to_text(file_path: &str) -> Result<String, SttError> {
    // Read the file into a byte vector.
    let mut file = File::open(file_path)?;
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)?;

    // Create a multipart form with the file and the required "model" parameter.
    let file_part = multipart::Part::bytes(file_bytes)
        .file_name("audio.wav")
        .mime_str("audio/wav")?; // Changed to WAV format

    let form = multipart::Form::new()
        .text("model", "whisper-1") // OpenAI's Whisper model
        .part("file", file_part);

    // Replace with your OpenAI API key.
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|err| SttError::EnvVarError(err))?;

    // Create a reqwest client and send the POST request.
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()
        .await?;

    // Check the status and print the response.
    let status = response.status();
    let text = response.text().await?;
    tracing::debug!("Status: {}\nResponse: {}", status, text);

    Ok(text)
}
