use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

use crate::commit_check::parse_commit::CommitHeader;

pub fn scope_enum(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.scope_enum;
    return (level, None);
}

pub fn scope_case(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.scope_case;
    return (level, None);
}

pub fn scope_empty(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.scope_empty;
    return (level, None);
}

pub fn scope_max_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.scope_max_length;
    return (level, None);
}

pub fn scope_min_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.scope_min_length;
    return (level, None);
}
