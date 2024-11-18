use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Fetch {
        /// Path to the local repository (default: current directory)
        #[arg(short, long)]
        repo: Option<String>,
    },
    Status {
        /// Path to the local repository (default: current directory)
        #[arg(short, long)]
        repo: Option<String>,
        /// Output the status in JSON format
        #[arg(short, long)]
        json: bool,
    }
}
