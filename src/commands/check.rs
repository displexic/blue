use clap::Args;

use crate::config::Config;
#[derive(Args, Debug)]
pub struct Check {
    /// The environment to check
    #[arg(short = 'e', long = "env")]
    env: Option<String>,
}

pub fn run(command: &Check, config: &Config) {
    println!("Checking environment: {:?}", command.env);
    println!(
        "Workspace Name: {:?}",
        config.workspace.as_ref().unwrap().name.as_ref().unwrap()
    );
}
