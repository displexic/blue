use blue_config::Config;
use blue_git::commit_check;
use clap::Args;

#[derive(Args, Debug)]
pub struct CommitCheck {
    /// The commit message to check
    #[arg(short = 'm', long = "message")]
    message: String,
}

pub fn run(command: &CommitCheck, config: &Config) {
    commit_check()
}
