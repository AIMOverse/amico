use amico::ai::service::Service;
use async_trait::async_trait;
use colored::Colorize;
use std::io::{self, Write};

use super::interface::{Task, TaskContext};

pub struct CliTask<S>
where
    S: Service,
{
    pub context: TaskContext<S>,
}

fn print_message_separator() {
    println!("--------------------");
}

#[async_trait]
impl<S> Task<S> for CliTask<S>
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
        Ok(CliTask { context })
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            println!("Enter your message");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.eq_ignore_ascii_case("quit") {
                println!("Exiting chatbot. Goodbye!");
                break;
            }

            print_message_separator();

            // Get response from AI service
            let response = match self.context.service.generate_text(input.to_string()).await {
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
