use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author("Mateo"), version("0.0.1"), about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add a new car")]
    Add {
        #[arg(short, long)]
        owner: String,

        #[arg(short, long)]
        plate: String,

        #[arg(short, long)]
        brand: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(Commands::Add {
        owner,
        plate,
        brand,
    }) = cli.command
    {
        println!("Adding a new car with details:");
        println!("Owner: {}", owner);
        println!("Plate: {}", plate);
        println!("Brand: {}", brand);
    }
}
