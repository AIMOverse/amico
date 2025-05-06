use std::{
    io::{self, Write},
    sync::Arc,
};

use colored::Colorize;
use evenio::prelude::*;
use tokio::{sync::Mutex, task::JoinHandle};

use super::events::UserContent;

/// A signal for output thread to inform input thread the output
/// task is complete.
struct OutputComplete;

/// A component representing STDIO interaction.
#[derive(Component)]
pub struct Stdio {
    tx: std::sync::mpsc::Sender<OutputComplete>,
    rx: Arc<Mutex<std::sync::mpsc::Receiver<OutputComplete>>>,
}

fn print_message_separator() {
    println!("--------------------");
}

impl Stdio {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    /// Spawn the user input event source.
    pub fn spawn_event_source<F>(&self, on_event: F) -> JoinHandle<()>
    where
        F: Fn(UserContent) + Send + 'static,
    {
        let rx = self.rx.clone();
        tokio::task::spawn_blocking(move || {
            println!();
            println!("{}", "I'm Amico, how can I assist you today?".green());
            print_message_separator();

            loop {
                print!("> ");

                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.eq_ignore_ascii_case("quit") {
                    // Exit the run loop
                    break;
                }

                print_message_separator();

                on_event(UserContent(input.to_string()));

                // Block until output completes
                {
                    rx.blocking_lock().recv().unwrap();
                }
            }

            print_message_separator();
            println!("{}", "Exiting chatbot. Goodbye!".green());
        })
    }

    pub fn print_message(&self, message: String) {
        println!("{}", "[Amico]".yellow());
        println!("{}", message.green());
        print_message_separator();

        // Inform event source to prompt user for the next input
        self.tx.send(OutputComplete).unwrap();
    }
}
