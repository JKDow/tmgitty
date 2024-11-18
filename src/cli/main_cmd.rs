use clap::{CommandFactory, Parser};

use crate::git::{fetch, GitStatus};

use super::sub_cmd::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run(self) {
        match self.command {
            Some(Commands::Fetch { repo }) => Cli::run_fetch(repo),
            Some(Commands::Status { repo, json }) => Cli::run_status(repo, json),
            None => {
                Cli::command().print_long_help().unwrap();
            }
        }
    }

    fn run_fetch(repo: Option<String>) {
        let repo = repo.unwrap_or_else(|| ".".to_string());
        match fetch(&repo) {
            Ok(()) => println!("Fetch completed successfully at {}", repo),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    fn run_status(repo: Option<String>, json: bool) {
        let repo = repo.unwrap_or_else(|| ".".to_string());
        match GitStatus::new(&repo) {
            Ok(status) => {
                if json {
                    println!("{}", status.json());
                } else {
                    println!("{}", status.status_line());
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
