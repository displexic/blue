use blue_config::git::commit_check::{CommitCheckConfig, RequiredCommitCheckRules};

mod merge_config;
mod parse_commit;
mod rules;

use parse_commit::{parse_commit, Commit};

pub fn run(commit_message: String, config: &CommitCheckConfig) {
    println!("checking commit: {}", commit_message);

    let commit: Commit = parse_commit(commit_message);

    let config: RequiredCommitCheckRules = merge_config::merge_preset_with_config(
        config.rules.as_ref().unwrap().clone(),
        &config.preset.as_ref().unwrap(),
    );

    // get config
    // convert commit message to commit struct
    // check every rule by passing the config and message to each rule function
}
