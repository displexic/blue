use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};
use regex::Regex;

pub mod body;
pub mod commit_type;
pub mod footer;
pub mod header;
pub mod scope;
pub mod subject;

pub fn references_empty<'a>(
    message: &String,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.references_empty;

    let issue_reference_regex = Regex::new(r"#\d+").unwrap();

    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !issue_reference_regex.is_match(&message) =>
        {
            Err((level, "References must not be empty.".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if issue_reference_regex.is_match(&message) =>
        {
            Err((level, "References must be empty.".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn signed_off_by<'a>(
    message: &String,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.signed_off_by;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !message.contains(value) =>
        {
            Err((
                level,
                "Commit message must contain 'Signed-off-by:'".to_string(),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if message.contains(value) => {
            Err((
                level,
                "Commit message must not contain 'Signed-off-by:'".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

pub fn trailer<'a>(
    message: &mut String,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.trailer;
    let binding = message.replace("\\\\", "\\");
    let sections: Vec<&str> = binding.split("\n").collect();
    let last_section = sections.last().unwrap_or(&"");
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !last_section.starts_with(value) =>
        {
            Err((
                level,
                "Commit message must end with 'Signed-off-by:'".to_string(),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if last_section.starts_with(value) =>
        {
            Err((
                level,
                "Commit message must not end with 'Signed-off-by:'".to_string(),
            ))
        }
        _ => Ok(()),
    }
}
