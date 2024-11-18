use clap::{CommandFactory, Parser};

use crate::git::{fetch, GitStatus};

use super::sub_cmd::{Commands, StatusCommand};

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
            Some(Commands::Status(status)) => Cli::run_status(status),
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

    fn run_status(cmd: StatusCommand) {
        let repo = cmd.repo.unwrap_or_else(|| ".".to_string());
        match GitStatus::new(&repo) {
            Ok(status) => {
                if cmd.json {
                    println!("{}", status.json());
                } else {
                    println!("{}", status.status_line(cmd.colors));
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
