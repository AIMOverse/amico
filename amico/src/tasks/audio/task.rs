use amico::ai::{provider::Provider, service::Service};
use async_trait::async_trait;
use colored::Colorize;
use std::io::{self, Write};

use crate::tasks::audio::{
    control::{playback, record_blocking},
    speech::{speech_to_text, text_to_speech},
};

use super::super::interface::{Task, TaskContext};

pub struct PulseAudioTask<S, P>
where
    S: Service<P>,
    P: Provider,
{
    pub context: TaskContext<S, P>,

    phantom: std::marker::PhantomData<P>,
}

fn print_message_separator() {
    println!("--------------------");
}

#[async_trait]
impl<S, P> Task<S, P> for PulseAudioTask<S, P>
where
    S: Service<P>,
    P: Provider,
{
    fn setup(context: TaskContext<S, P>) -> Result<Self, Box<dyn std::error::Error>> {
        // Print global prompt
        println!();
        println!(
            "{}",
            "I'm Amico, your personal AI assistant. How can I assist you today?".green()
        );
        print_message_separator();
        Ok(PulseAudioTask {
            context,
            phantom: std::marker::PhantomData,
        })
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
                break;
            }

            // Record user's voice into a file
            record_blocking("cache/user.wav")?;

            // Convert sound file to text
            let stt_res = speech_to_text("cache/user.wav").await?;

            println!("[User] {}", stt_res);

            print_message_separator();

            // Get response from AI service
            let response = match self
                .context
                .service
                .generate_text(stt_res.to_string())
                .await
            {
                Ok(response) => {
                    // Convert response text to audio
                    text_to_speech(&response, "cache/assistant.wav").await?;

                    // Play the sound in a separate thread
                    if let Err(err) = playback("cache/assistant.wav").await {
                        tracing::error!("Error playing audio: {err}");
                        eprintln!("Error playing audio");
                    }

                    response
                }
                Err(err) => {
                    eprintln!("Error generating text: {err}");
                    continue;
                }
            };
            println!("{}", "[Amico]".yellow());
            println!("{}", response.green());
            print_message_separator();
        }

        Ok(())
    }
}
