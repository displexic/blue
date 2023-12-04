use blue_config::git::commit_check::CommitCheckRules;

pub fn footer_leading_blank(
    footer: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn footer_full_stop(
    footer: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn footer_empty(footer: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn footer_max_length(
    footer: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn footer_min_length(
    footer: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
