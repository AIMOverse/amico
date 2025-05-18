use std::{
    future::Future,
    io::{self, Write},
    sync::Arc,
};

use amico_core::{
    ecs::Component,
    traits::EventSource,
    types::{AgentEvent, EventContent},
};
use colored::Colorize;
use tokio::sync::{mpsc, Mutex};

use crate::engine::events::ConsoleInput;

/// Create a new STDIO module.
pub fn create_cli_client() -> (CliComponent, CliEventSource) {
    let (tx, rx) = mpsc::channel(1);
    (CliComponent::new(tx), CliEventSource::new(rx))
}

/// A signal for output thread to inform input thread the output
/// task is complete.
struct OutputComplete;

/// A component representing STDIO interaction.
#[derive(Component)]
pub struct CliComponent {
    tx: mpsc::Sender<OutputComplete>,
}

/// An event source for STDIO interaction.
pub struct CliEventSource {
    rx: Arc<Mutex<mpsc::Receiver<OutputComplete>>>,
}

fn print_message_separator() {
    println!("--------------------");
}

impl CliComponent {
    fn new(tx: mpsc::Sender<OutputComplete>) -> Self {
        Self { tx }
    }

    pub fn print_message(&self, message: &str) {
        println!("{}", "[Amico]".yellow());
        println!("{}", message.green());
        print_message_separator();

        // Inform event source to prompt user for the next input
        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send(OutputComplete).await.unwrap();
        });
    }

    pub fn handle_record_start(&self) {
        println!("{}", "Recording started. Press any key to finish.".yellow());
        let tx = self.tx.clone();
        tokio::spawn(async move {
            tx.send(OutputComplete).await.unwrap();
        });
    }

    pub fn handle_record_finish(&self) {
        println!("{}", "Recording finished".yellow());
    }

    pub fn handle_playback_finish(&self) {
        println!("{}", "Playback finished".yellow());
    }
}

impl CliEventSource {
    fn new(rx: mpsc::Receiver<OutputComplete>) -> Self {
        Self {
            rx: Arc::new(Mutex::new(rx)),
        }
    }
}

impl EventSource for CliEventSource {
    async fn run<F, Fut>(&self, on_event: F) -> anyhow::Result<()>
    where
        F: Fn(amico_core::types::AgentEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        println!();
        println!("{}", "Type \"s\" to speak to Amico, \"q\" to quit".blue());
        println!("{}", "I'm Amico, how can I assist you today?".green());
        print_message_separator();

        loop {
            print!("> ");

            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("q") {
                // Exit the run loop
                break;
            }

            print_message_separator();

            on_event(AgentEvent::new(
                "ConsoleInput",
                "Stdio",
                Some(EventContent::Content(
                    serde_json::to_value(ConsoleInput(input.to_string())).unwrap(),
                )),
                None,
            ))
            .await;

            // Wait until output completes
            let mut rx = self.rx.lock().await;

            tracing::debug!("Waiting for output complete");
            rx.recv().await.unwrap();
        }

        print_message_separator();
        println!("{}", "Exiting chatbot. Goodbye!".green());

        Ok(())
    }
}
