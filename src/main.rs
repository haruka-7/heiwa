use clap::{Parser, Subcommand};
use serve::serve;

mod handlers;
mod serve;

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

    if let Some(init) = cli.init.as_deref() {
        println!("Value for init: {init}");
    } else {
        match &cli.command {
            Some(Commands::Serve) => serve().await,
            None => {}
        }
    }
}
