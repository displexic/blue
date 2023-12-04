use blue_config::git::commit_check::CommitCheckRules;

pub fn body_full_stop(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_leading_blank(
    body: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_empty(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_max_length(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_min_length(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_max_line_length(
    body: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_min_line_length(
    body: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_max_lines(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}

pub fn body_case(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<&'static str>) {
    return (true, Some("error"));
}
