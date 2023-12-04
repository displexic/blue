use blue_config::git::commit_check::{Level, RequiredCommitCheckRules};

pub mod body;
pub mod commit_type;
pub mod footer;
pub mod header;
pub mod scope;
pub mod subject;

/// Not implemented yet.
pub fn references_empty(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule) = rules.references_empty;
    return (level, None);
}

pub fn signed_off_by(
    body: Vec<String>,
    rules: RequiredCommitCheckRules,
) -> (Level, Option<String>) {
    let (level, rule, value) = rules.signed_off_by;
    return (level, None);
}

pub fn trailer(body: Vec<String>, rules: RequiredCommitCheckRules) -> (Level, Option<String>) {
    let (level, rule, value) = rules.trailer;
    return (level, None);
}
