use clap::Parser;
use tmgitty::cli::Cli;

fn main() {
    Cli::parse().run();
}
