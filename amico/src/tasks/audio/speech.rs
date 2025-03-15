#[derive(Debug, thiserror::Error)]
pub enum TtsError {
    #[error("Failed to read audio file")]
    ReadError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SttError {
    #[error("Failed to read audio file")]
    ReadError(#[from] std::io::Error),
}

pub async fn text_to_speech(text: &str, filename: &str) -> Result<(), TtsError> {
    println!("tts text: {}, file: {}", text, filename);
    Ok(())
}

pub async fn speech_to_text(filename: &str) -> Result<String, SttError> {
    println!("stt file: {}", filename);
    Ok("TTS_OUTPUT".to_string())
}
