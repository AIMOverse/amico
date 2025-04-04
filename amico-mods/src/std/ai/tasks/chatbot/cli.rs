use amico::{ai::services::CompletionService, task::Task};
use async_trait::async_trait;
use colored::Colorize;
use std::io::{self, Write};

use crate::std::ai::{providers::RigProvider, services::InMemoryService};

use super::context::ChatbotContext;

/// A task that runs a chatbot in the CLI.
#[derive(Debug)]
pub struct CliTask;

/// Errors that may occur during chatbot task
#[derive(Debug, thiserror::Error)]
pub enum CliTaskError {}

fn print_message_separator() {
    println!("--------------------");
}

#[async_trait]
impl Task for CliTask {
    type Context = ChatbotContext<InMemoryService<RigProvider>>;
    type Error = CliTaskError;

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
            println!("Enter your message");
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

            // Get response from AI service
            let response = match context.service.generate_text(input.to_string()).await {
                Ok(response) => response,
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
