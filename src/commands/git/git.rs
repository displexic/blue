use clap::{Args, Subcommand};

use super::commit_check::CommitCheck;

#[derive(Args, Debug)]
pub struct GitArgs {
    // Checks if the commit message is valid
    #[clap(subcommand)]
    pub commands: GitCommands,
}

#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Checks if the commit message is valid
    #[clap(name = "commit-check")]
    CommitCheck(CommitCheck),
}
