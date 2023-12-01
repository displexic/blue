use clap::Args;

pub mod check;

#[derive(Args, Debug)]
pub struct Check {
    /// The environment to check
    #[arg(short = 'e', long = "env")]
    env: Option<String>,
}
