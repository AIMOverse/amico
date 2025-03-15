use reqwest::multipart;
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum TtsError {
    #[error("Failed to read audio file")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to initialize TTS engine")]
    TtsInitError(String),

    #[error("Failed to synthesize speech")]
    TtsSynthesisError(String),

    #[error("Piper binary not found")]
    PiperNotFound,

    #[error("Piper voice model not found")]
    VoiceModelNotFound,
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

    // Define paths to Piper binary and voice model
    // These paths should be configured based on your deployment environment
    let piper_binary =
        std::env::var("PIPER_BINARY").unwrap_or_else(|_| "~/piper/piper".to_string());
    let piper_model = std::env::var("PIPER_MODEL")
        .unwrap_or_else(|_| "~/piper/voices/zh_CN-huayan-medium.onnx".to_string());

    // Expand tilde in paths if present
    let piper_binary = if piper_binary.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            piper_binary.replacen("~", &home, 1)
        } else {
            piper_binary
        }
    } else {
        piper_binary
    };

    let piper_model = if piper_model.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            piper_model.replacen("~", &home, 1)
        } else {
            piper_model
        }
    } else {
        piper_model
    };

    // Check if the binary and model exist
    if !Path::new(&piper_binary).exists() {
        return Err(TtsError::PiperNotFound);
    }

    if !Path::new(&piper_model).exists() {
        return Err(TtsError::VoiceModelNotFound);
    }

    // Clone the text and file_path to own the data before moving into the closure
    let text_owned = text.to_string();
    let file_path_owned = file_path.to_string();

    // Use tokio spawn_blocking for process execution
    tokio::task::spawn_blocking(move || {
        // Create a temporary file for the input text
        let temp_dir = std::env::temp_dir();
        let input_file = temp_dir.join("piper_input.txt");
        std::fs::write(&input_file, &text_owned)?;

        // Run Piper as a subprocess
        let output = Command::new(&piper_binary)
            .arg("--model")
            .arg(&piper_model)
            .arg("--output_file")
            .arg(&file_path_owned)
            .arg("--input_file")
            .arg(input_file)
            .output()?;

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(TtsError::TtsSynthesisError(error_message));
        }

        Ok(())
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
