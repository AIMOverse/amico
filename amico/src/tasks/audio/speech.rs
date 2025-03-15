use reqwest::multipart;
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, thiserror::Error)]
pub enum TtsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to synthesize speech")]
    TtsSynthesisError(String),

    #[error("API request error: {0}")]
    ApiRequestError(#[from] reqwest::Error),

    #[error("API key not found")]
    ApiKeyNotFound(#[from] std::env::VarError),
}

#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("Failed to get API key")]
    EnvVarError(#[from] std::env::VarError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to create multipart request")]
    MultipartError(#[from] reqwest::Error),

    #[error("Failed to set mime type")]
    MimeError(#[from] reqwest::header::InvalidHeaderValue),
}

pub async fn text_to_speech(text: &str, file_path: &str) -> Result<(), TtsError> {
    tracing::debug!("tts text: {}, output file: {}", text, file_path);

    // Get OpenAI API key from environment variable
    let api_key = std::env::var("OPENAI_API_KEY").map_err(|e| TtsError::ApiKeyNotFound(e))?;

    // Define request body for OpenAI's text-to-speech API
    #[derive(serde::Serialize)]
    struct TtsRequest {
        model: String,
        input: String,
        voice: String,
        response_format: String,
        speed: f32,
    }

    let request_body = TtsRequest {
        model: "tts-1".to_string(), // Can be upgraded to tts-1-hd for higher quality
        input: text.to_string(),
        voice: "alloy".to_string(), // Options: alloy, echo, fable, onyx, nova, shimmer
        response_format: "mp3".to_string(),
        speed: 1.0,
    };

    // Create a reqwest client and send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/audio/speech")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| TtsError::ApiRequestError(e))?;

    // Check if the request was successful
    if !response.status().is_success() {
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(TtsError::TtsSynthesisError(format!(
            "OpenAI API error: {}",
            error_message
        )));
    }

    // Get the audio bytes from the response
    let audio_bytes = response
        .bytes()
        .await
        .map_err(|e| TtsError::ApiRequestError(e))?;

    // Create the directory for the output file if it doesn't exist
    if let Some(parent) = Path::new(file_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| TtsError::IoError(e))?;
    }

    // Write the audio bytes to the file
    std::fs::write(file_path, audio_bytes).map_err(|e| TtsError::IoError(e))?;

    Ok(())
}

pub async fn speech_to_text(file_path: &str) -> Result<String, SttError> {
    // Read the file into a byte vector.
    let mut file = File::open(file_path)?;
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)?;

    // Create a multipart form with the file and the required "model" parameter.
    let file_part = multipart::Part::bytes(file_bytes)
        .file_name("audio.mp3")
        .mime_str("audio/mpeg")?; // Changed to MP3 format

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

    // Response schema of OpenAI's Whisper API
    #[derive(serde::Deserialize)]
    struct WhisperResponse {
        text: String,
    }

    // Check the status and print the response.
    let status = response.status();
    let text = response.json::<WhisperResponse>().await?.text;
    tracing::debug!("Status: {}\nResponse: {}", status, text);

    Ok(text)
}
