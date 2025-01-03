mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version("0.0.1"), about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add(commands::Add),
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Add(commands::Add {
        owner,
        plate,
        brand,
    })) = cli.command
    {
        println!("Adding a new car with details:");
        println!("Owner: {}", owner);
        println!("Plate: {}", plate);
        println!("Brand: {}", brand);
    }
}

#[test]
fn verify_app() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
