use std::io::{self, Write};

fn print_demo_hint() {
    println!("THIS IS ONLY A DEMO VERSION OF AMICO");
    println!();
    println!("CHECKOUT OUR DOCS FOR MORE INFORMATION:");
    println!("https://www.amico.dev");
    println!();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_demo_hint();
    loop {
        print!("Enter your message: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            println!("Exiting chatbot. Goodbye!");
            break;
        }

        // Mock response from AI service
        let response = "This is a mock response from the AI service.";
        println!("AI: {}", response);
    }

    Ok(())
}
