use blue_config::git::commit_check::CommitCheckRules;

pub mod body;
pub mod commit_type;
pub mod footer;
pub mod header;
pub mod scope;
pub mod subject;

type CommitCheckBuilder =
    fn(message: String, config: CommitCheckRules) -> (bool, Option<&'static str>);

pub mod CommitCheck {
    use super::CommitCheckBuilder;
    pub fn builder(callback: CommitCheckBuilder) {}
}

pub fn references_empty() {
    return CommitCheck::builder(|message, config| {
        return (true, Some("Hello world"));
    });
}

pub fn signed_off_by() {}

pub fn trailer() {}
