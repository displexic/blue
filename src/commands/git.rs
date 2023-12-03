use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct GitArgs {
    // Checks if the commit message is valid
    #[clap(name = "check-commit")]
    #[clap(subcommand)]
    pub commands: GitCommands,
}

#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Checks if the commit message is valid
    CheckCommit(CheckCommit),
}

#[derive(Args, Debug)]
pub struct CheckCommit {}

pub mod check {
    pub fn run() {
        println!("Hello world")
    }
}
