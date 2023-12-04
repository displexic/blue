use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

pub fn body_full_stop(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.body_full_stop;
    return (level, None);
}

pub fn body_leading_blank(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.body_leading_blank;
    return (level, None);
}

pub fn body_empty(body: Vec<String>, rules: RequiredCommitCheckRules) -> (Level, Option<String>) {
    let (level, rule) = rules.body_empty;
    return (level, None);
}

pub fn body_max_length(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_max_length;
    return (level, None);
}

pub fn body_min_length(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_min_length;
    return (level, None);
}

pub fn body_max_line_length(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_max_line_length;
    return (level, None);
}

pub fn body_min_line_length(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_min_line_length;
    return (level, None);
}

pub fn body_max_lines(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_max_lines;
    return (level, None);
}

pub fn body_case(body: Vec<String>, rules: RequiredCommitCheckRules) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_case;
    return (level, None);
}
