use blue_config::git::commit_check::CommitCheckRules;

pub mod body;
pub mod commit_type;
pub mod footer;
pub mod header;
pub mod scope;
pub mod subject;

/// Not implemented yet.
pub fn references_empty(
    body: Vec<String>,
    rules: CommitCheckRules,
) -> (bool, Option<&'static str>) {
    return (false, None);
}

pub fn signed_off_by(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<String>) {
    return (true, Some("error".to_string()));
}

pub fn trailer(body: Vec<String>, rules: CommitCheckRules) -> (bool, Option<String>) {
    return (true, Some("error".to_string()));
}
