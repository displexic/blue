use blue_config::git::commit_check::CommitCheckRules;

use crate::commit_check::parse_commit::CommitHeader;

pub fn scope_enum(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn scope_case(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn scope_empty(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn scope_max_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn scope_min_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
