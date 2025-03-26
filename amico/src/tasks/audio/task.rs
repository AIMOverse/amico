use amico::ai::service::Service;
use async_trait::async_trait;
use colored::Colorize;
use std::io::{self, Write};

use crate::tasks::audio::{
    control::{playback, record_blocking},
    speech::{speech_to_text, text_to_speech},
};

use super::super::interface::{Task, TaskContext};

pub struct AudioChatTask<S>
where
    S: Service,
{
    pub context: TaskContext<S>,
}

fn print_message_separator() {
    println!("--------------------");
}

#[async_trait]
impl<S> Task<S> for AudioChatTask<S>
where
    S: Service,
{
    fn setup(context: TaskContext<S>) -> Result<Self, Box<dyn std::error::Error>> {
        // Print global prompt
        println!();
        println!(
            "{}",
            "I'm Amico, your personal AI assistant. How can I assist you today?".green()
        );
        print_message_separator();
        Ok(AudioChatTask { context })
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
            record_blocking("cache/user.mp3").await?;
            tracing::info!("Recorded user's voice");

            // Convert sound file to text
            let stt_res = speech_to_text("cache/user.mp3").await?;
            tracing::info!("Converted sound file to text");

            println!("[User] {}", stt_res);

            print_message_separator();

            // Get response from AI service
            match self
                .context
                .service
                .generate_text(stt_res.to_string())
                .await
            {
                Ok(response) => {
                    println!("{}", "[Amico]".yellow());
                    println!("{}", response.green());

                    // Convert response text to audio
                    tracing::info!("Converting response text to audio");
                    text_to_speech(&response, "./cache/assistant.mp3").await?;
                    tracing::info!("Converted response text to audio");

                    // Play the sound in a separate thread
                    if let Err(err) = playback("cache/assistant.mp3").await {
                        tracing::error!("Error playing audio: {err}");
                        eprintln!("Error playing audio");
                    }
                }
                Err(err) => {
                    eprintln!("Error generating text: {err}");
                    continue;
                }
            };

            print_message_separator();
        }
    }
}
