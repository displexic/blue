//! Named commit_type, because type is a reserved keyword.

use blue_config::git::commit_check::CommitCheckRules;

use crate::commit_check::parse_commit::CommitHeader;

pub fn type_enum(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn type_case(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn type_empty(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn type_max_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn type_min_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
