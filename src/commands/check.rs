use clap::Args;

use blue_config::Config;
#[derive(Args, Debug)]
pub struct CheckArgs {
    /// The environment to check
    #[arg(short = 'e', long = "env")]
    env: Option<String>,
}

pub fn run(command: &CheckArgs, config: &Config) {
    tracing::info!("Checking environment: {:?}", command.env);
    tracing::info!(
        "Workspace Name: {:?}",
        config.workspace.as_ref().unwrap().name.as_ref().unwrap()
    );
}
