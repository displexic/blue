//! Named commit_type, because type is a reserved keyword.

use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

use crate::commit_check::parse_commit::CommitHeader;

pub fn type_enum(header: CommitHeader, rules: RequiredCommitCheckRules) -> (Level, Option<String>) {
    let (level, rule, value) = rules.type_enum;
    return (level, None);
}

pub fn type_case(header: CommitHeader, rules: RequiredCommitCheckRules) -> (Level, Option<String>) {
    let (level, rule, value) = rules.type_case;
    return (level, None);
}

pub fn type_empty(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.type_empty;
    return (level, None);
}

pub fn type_max_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.type_max_length;
    return (level, None);
}

pub fn type_min_length(
    header: CommitHeader,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.type_min_length;
    return (level, None);
}
