mod commands;

/// core module containing useful traits
mod core;
use core::*;

/// store module containing the data store
mod store;

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

    match cli.command {
        Some(Commands::Add(add)) => add.call(),
        None => println!("No command provided"),
    };
}

#[test]
fn verify_app() {
    use clap::CommandFactory;

    Cli::command().debug_assert();
}
