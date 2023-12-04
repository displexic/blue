use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

pub fn footer_leading_blank(
    footer: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.footer_leading_blank;
    return (level, None);
}

pub fn footer_empty(
    footer: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.footer_empty;
    return (level, None);
}

pub fn footer_max_length(
    footer: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.footer_max_length;
    return (level, None);
}

pub fn footer_max_line_length(
    footer: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.body_max_line_length;
    return (level, None);
}
