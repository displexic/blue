use blue_config::git::commit_check::{CommitCheckRules, RequiredCommitCheckRules};

pub fn merge_preset_with_config(
    optional_config: Option<CommitCheckRules>,
    preset: RequiredCommitCheckRules,
) -> RequiredCommitCheckRules {
    let config = match optional_config {
        Some(value) => value,
        None => return preset,
    };

    let merged_config = preset;

    merged_config
}
