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
    Serve {
        /// Define port number to use whith the builtin server
        #[arg(short, long)]
        port: Option<u16>,
        /// Define request timeout in seconds
        #[arg(short, long)]
        timeout: Option<u64>,
    },
    /// Create a markdown file with empty metadatas
    Page {
        /// Markdown extension (.md) is automatically added
        #[clap(default_value = "newpage", value_name = "PAGE_NAME")]
        name: String,
    },
}
