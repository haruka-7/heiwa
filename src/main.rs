use clap::Parser;
use cli::commands::{Cli, Commands};
use cli::init::init;
use cli::page::page;
use cli::serve::serve;

mod cli;
mod configuration;
mod entities;
mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            init(name);
        }
        Commands::Serve { port, timeout } => serve(port, timeout).await,
        Commands::Page { name } => {
            page(name);
        }
    }
}
