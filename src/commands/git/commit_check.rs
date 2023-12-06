use blue_config::{
    git::{commit_check::CommitCheckConfig, GitConfig},
    Config,
};
use blue_git::commit_check;
use clap::Args;
use serde_json::Value;

#[derive(Args, Debug)]
pub struct CommitCheckArgs {
    /// The commit message to check
    #[arg(short = 'm', long = "message")]
    message: String,
}

fn get_git_config(config: &Config) -> &GitConfig {
    match &config.git {
        Some(value) => value,
        None => {
            tracing::error!("No git config found in blue.toml");
            std::process::exit(1);
        }
    }
}

fn get_git_check_commit_config(config: &Config) -> &CommitCheckConfig {
    match &get_git_config(config).commit_check {
        Some(value) => value,
        None => {
            tracing::error!("No commit check config found in blue.toml");
            std::process::exit(1);
        }
    }
}

pub fn run(command: &CommitCheckArgs, config: &Config) {
    let git_commit_check_config = get_git_check_commit_config(config);

    // If a preset is provided, then continue regardless of whether rules are provided.
    // Otherwise, ensure that at least one rule is provided, otherwise exit.
    match &git_commit_check_config.preset {
        Some(value) => Some(value),
        None => {
            // Serialize the struct into JSON
            let config_rule_json = serde_json::to_value(
                &git_commit_check_config.rules.as_ref().unwrap_or_else(|| {
                    tracing::error!("No commit check rules found in blue.toml");
                    std::process::exit(1);
                }),
            )
            .unwrap();

            // Count non-null fields in JSON
            let config_rule_count = match config_rule_json {
                Value::Object(map) => map.values().filter(|v| !v.is_null()).count(),
                _ => 0,
            };

            if config_rule_count == 0 {
                tracing::error!("No commit check rules found in blue.toml");
                std::process::exit(1);
            }

            None
        }
    };

    commit_check::run(command.message.clone(), git_commit_check_config);
}
