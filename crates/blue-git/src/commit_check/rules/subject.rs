use blue_config::git::commit_check::CommitCheckRules;

use crate::commit_check::parse_commit::CommitHeader;

pub fn subject_case(header: CommitHeader, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn subject_empty(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn subject_full_stop(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn subject_max_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn subject_min_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn subject_exclamation(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
