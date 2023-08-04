use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CheckWkb(CheckWkbArgs),
}

#[derive(Args)]
struct CheckWkbArgs {
    bytes: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CheckWkb(args) => {
            println!("Check WKB success");
        }
    }
}
