use clap::Parser;
use cli::cli::{Cli, Commands};
use cli::init::init;
use cli::serve::serve;

mod cli;
mod handlers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            init(name);
        },
        Commands::Serve => {
            serve().await
        },
    }
}
