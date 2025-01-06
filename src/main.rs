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
    List(commands::List),
    Reset(commands::Reset),
    NextRevision(commands::NextRevision),
    MarkRevision(commands::MarkRevision),
    NextRoadTax(commands::NextRoadTax),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add(add)) => add.call_with_output(),
        Some(Commands::List(list)) => list.call_with_output(),
        Some(Commands::Reset(reset)) => reset.call_with_output(),
        Some(Commands::NextRevision(next_revision)) => next_revision.call_with_output(),
        Some(Commands::MarkRevision(mark_revision)) => mark_revision.call_with_output(),
        Some(Commands::NextRoadTax(next_road_tax)) => next_road_tax.call_with_output(),
        None => println!("No command provided"),
    };
}

#[test]
fn verify_app() {
    use clap::CommandFactory;

    Cli::command().debug_assert();
}
