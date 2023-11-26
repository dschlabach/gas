use clap::Parser;
use dotenv;

#[derive(Parser)]
#[command(
    author = "Daniel Schlabach",
    about = "A CLI for getting gas prices across different EVM chains."
)]
struct Args {}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let args = Args::parse();

    return Ok(());
}
