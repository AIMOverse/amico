use colored::Colorize;
use evenio::prelude::*;

#[derive(Component)]
pub struct StdioOutputAction;

fn print_message_separator() {
    println!("--------------------");
}

impl StdioOutputAction {
    pub fn print_message(&self, message: String) {
        println!("{}", "[Amico]".yellow());
        println!("{}", message.green());
        print_message_separator();
    }
}
