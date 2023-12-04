use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

use crate::commit_check::parse_commit::CommitHeader;

pub fn subject_case(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.subject_case;
    return (level, None);
}

pub fn subject_empty(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.subject_empty;
    return (level, None);
}

pub fn subject_full_stop(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.subject_full_stop;
    return (level, None);
}

pub fn subject_max_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.subject_max_length;
    return (level, None);
}

pub fn subject_min_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.subject_min_length;
    return (level, None);
}

pub fn subject_exclamation(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.subject_exclamation;
    return (level, None);
}
