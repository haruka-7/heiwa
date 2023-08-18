use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create an heiwa website project
    Init {
        #[clap(default_value = ".", value_name = "PROJECT_NAME")]
        name: String,
    },
    /// Launch the webserver on localhost to access the website
    Serve,
}
