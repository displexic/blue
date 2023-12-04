use blue_config::git::commit_check::CommitCheckRules;

use crate::commit_check::parse_commit::CommitHeader;

pub fn header_header_case(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn header_full_stop(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn header_max_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn header_min_length(
    header: CommitHeader,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
