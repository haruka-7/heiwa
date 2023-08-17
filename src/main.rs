use clap::{Parser, Subcommand};
use init::init;
use serve::serve;

mod handlers;
mod serve;
mod init;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Create an heiwa website project
    #[arg(short, long, value_name = "PROJECT_NAME")]
    init: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the webserver on localhost to access the website
    Serve,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    if let Some(project_name) = cli.init.as_deref() {
        init(project_name.to_string());
    } else {
        match &cli.command {
            Some(Commands::Serve) => serve().await,
            None => {}
        }
    }
}
