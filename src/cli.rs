use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
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
