use std::io::{self, Write};

use colored::Colorize;
use evenio::prelude::*;
use tokio::task::JoinHandle;

use super::events::UserContent;

#[derive(Component)]
pub struct StdioEventSource;

fn print_message_separator() {
    println!("--------------------");
}

impl StdioEventSource {
    pub fn spawn<F>(&self, on_event: F) -> JoinHandle<()>
    where
        F: Fn(UserContent) + Send + 'static,
    {
        tokio::task::spawn_blocking(move || {
            println!();
            println!(
                "{}",
                "I'm Amico, your personal AI assistant. How can I assist you today?".green()
            );
            print_message_separator();

            loop {
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input.eq_ignore_ascii_case("quit") {
                    // Exit the run loop
                    break;
                }

                print_message_separator();

                on_event(UserContent(input.to_string()))
            }

            println!("{}", "Exiting chatbot. Goodbye!".green());
        })
    }
}
