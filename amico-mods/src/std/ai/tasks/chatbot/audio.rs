use amico::{ai::service::Service, task::Task};
use amico_hal::{
    interface::audio::{AudioPlayer, AudioRecorder},
    os::common::audio::{AudioDriver, AudioPlaybackError, AudioRecordingError},
};
use async_trait::async_trait;
use colored::Colorize;
use std::io::{self, Write};

use crate::std::ai::{
    providers::RigProvider,
    services::InMemoryService,
    tasks::chatbot::speech::{speech_to_text, text_to_speech},
};

use super::{
    context::ChatbotContext,
    speech::{SttError, TtsError},
};

pub struct AudioChatTask;

fn print_message_separator() {
    println!("--------------------");
}

#[derive(Debug, thiserror::Error)]
pub enum AudioChatTaskError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TTS error: {0}")]
    TtsError(#[from] TtsError),

    #[error("STT error: {0}")]
    SttError(#[from] SttError),

    #[error("Recorder driver error: {0}")]
    RecorderDriverError(#[from] AudioRecordingError),

    #[error("Player driver error: {0}")]
    PlayerDriverError(#[from] AudioPlaybackError),
}

#[async_trait]
impl Task for AudioChatTask {
    type Context = ChatbotContext<InMemoryService<RigProvider>>;
    type Error = AudioChatTaskError;

    async fn before_run(&mut self, _context: &mut Self::Context) -> Result<(), Self::Error> {
        // Print global prompt
        println!();
        println!(
            "{}",
            "I'm Amico, your personal AI assistant. How can I assist you today?".green()
        );
        print_message_separator();
        Ok(())
    }

    async fn after_run(&mut self, _context: &mut Self::Context) -> Result<(), Self::Error> {
        println!("{}", "Exiting chatbot. Goodbye!".green());
        Ok(())
    }

    async fn run(&mut self, context: &mut Self::Context) -> Result<(), Self::Error> {
        loop {
            println!("Press enter then say something...");
            println!("(or type 'quit' to exit)");
            print!("> ");
            io::stdout().flush()?;

            // Block until enter is pressed
            let mut stdin_input = String::new();
            io::stdin().read_line(&mut stdin_input)?;

            if stdin_input.trim().eq_ignore_ascii_case("quit") {
                println!("Exiting chatbot. Goodbye!");
                return Ok(());
            }

            // Record user's voice into a file
            AudioDriver::record("cache/user.mp3").await?;
            tracing::info!("Recorded user's voice");

            // Convert sound file to text
            let stt_res = speech_to_text("cache/user.mp3").await?;
            tracing::info!("Converted sound file to text");

            println!("[User] {}", stt_res);

            print_message_separator();

            // Get response from AI service
            match context.service.generate_text(stt_res.to_string()).await {
                Ok(response) => {
                    println!("{}", "[Amico]".yellow());
                    println!("{}", response.green());

                    // Convert response text to audio
                    tracing::info!("Converting response text to audio");
                    text_to_speech(&response, "./cache/assistant.mp3").await?;
                    tracing::info!("Converted response text to audio");

                    // Play the sound in a separate thread
                    if let Err(err) = AudioDriver::play("cache/assistant.mp3").await {
                        tracing::error!("Error playing audio: {err}");
                        eprintln!("Error playing audio");
                    }
                }
                Err(err) => {
                    tracing::error!("Error generating text: {err}");
                    eprintln!("Error generating text");
                    continue;
                }
            };

            print_message_separator();
        }
    }
}
