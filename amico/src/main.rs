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
    Ok(())
}
