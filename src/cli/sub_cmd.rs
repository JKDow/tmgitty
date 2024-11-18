use clap::{ArgGroup, Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Fetch {
        /// Path to the local repository (default: current directory)
        #[arg(short, long)]
        repo: Option<String>,
    },
    Status(StatusCommand),
}

#[derive(Parser)]
#[command(group(
    ArgGroup::new("CommandOptions")
        .required(false)
        .args(&["colors", "json"]),
))]
pub struct StatusCommand {
    #[arg(short, long)]
    pub repo: Option<String>,
    #[arg(short, long)]
    pub colors: bool,
    #[arg(short, long)]
    pub json: bool,
}
