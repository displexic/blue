//! Named commit_type, because type is a reserved keyword.

use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

use crate::commit_check::{find_case::find_case, parse_commit::CommitHeader};

pub fn type_enum<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.type_enum;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !value.contains(&header.commit_type) =>
        {
            Err((
                level,
                format!("Type must be one of the following: {:?}", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if value.contains(&header.commit_type) =>
        {
            Err((
                level,
                format!("Type must not be one of the following: {:?}", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn type_case<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.type_case;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !find_case(&header.commit_type, value) =>
        {
            Err((
                level,
                format!("Type must be in one of the following cases: {:?}", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if find_case(&header.commit_type, value) =>
        {
            Err((
                level,
                format!("Type must not be any of the following cases: {:?}", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn type_empty<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.type_empty;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !header.commit_type.is_empty() =>
        {
            Err((level, "Type must be empty".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if header.commit_type.is_empty() =>
        {
            Err((level, "Type must not be empty".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn type_max_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.type_max_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if header.commit_type.len() > *value =>
        {
            Err((
                level,
                format!("Type must not be longer than {} characters", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if header.commit_type.len() <= *value =>
        {
            Err((
                level,
                format!("Type must be longer than {} characters", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn type_min_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.type_min_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if header.commit_type.len() < *value =>
        {
            Err((
                level,
                format!("Type must not be shorter than {} characters", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if header.commit_type.len() >= *value =>
        {
            Err((
                level,
                format!("Type must be shorter than {} characters", value),
            ))
        }
        _ => Ok(()),
    }
}
