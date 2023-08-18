use clap::Parser;
use cli::{Cli, Commands};
use init::init;
use serve::serve;

mod cli;
mod handlers;
mod init;
mod serve;

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
