use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

use crate::commit_check::{find_case::find_case, parse_commit::CommitHeader};

pub fn header_case<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.header_case;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if !find_case(&header.content, value) {
                return Err((
                    level,
                    format!("Header must be in one of the following cases: {:?}", value),
                ));
            }
            Ok(())
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) => {
            if find_case(&header.content, value) {
                return Err((
                    level,
                    format!("Header must not be any of the following cases: {:?}", value),
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn header_full_stop<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.header_full_stop;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !header.subject.ends_with('.') =>
        {
            Err((level, "Header should end with a full stop.".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if header.subject.ends_with('.') =>
        {
            Err((level, "Header should not end with a full stop.".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn header_max_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.header_max_length;
    match (level, rule, value) {
        (Level::Warning, Rule::Always, _) | (Level::Error, Rule::Always, _)
            if &header.content.len() > value =>
        {
            Err((
                level,
                format!("Header must not be more than {} characters long", value),
            ))
        }
        (Level::Warning, Rule::Never, _) | (Level::Error, Rule::Never, _)
            if &header.content.len() < value =>
        {
            Err((
                level,
                format!("Header must be more than {} characters long", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn header_min_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.header_min_length;
    match (level, rule, value) {
        (Level::Warning, Rule::Always, _) | (Level::Error, Rule::Always, _)
            if &header.content.len() < value =>
        {
            Err((
                level,
                format!("Header must not be less than {} characters long", value),
            ))
        }
        (Level::Warning, Rule::Never, _) | (Level::Error, Rule::Never, _)
            if &header.content.len() > value =>
        {
            Err((
                level,
                format!("Header must be less than {} characters long", value),
            ))
        }
        _ => Ok(()),
    }
}
