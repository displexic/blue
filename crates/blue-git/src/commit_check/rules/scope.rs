use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

use crate::commit_check::{find_case::find_case, parse_commit::CommitHeader};

pub fn scope_enum<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.scope_enum;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !value.contains(&header.scope) =>
        {
            Err((
                level,
                format!("Scope must be one of the following: {:?}", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if value.contains(&header.scope) =>
        {
            Err((
                level,
                format!("Scope must not be one of the following: {:?}", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn scope_case<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.scope_case;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !find_case(&header.scope, value) =>
        {
            Err((
                level,
                format!("Scope must be in one of the following cases: {:?}", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if find_case(&header.scope, value) =>
        {
            Err((
                level,
                format!("Scope must not be any of the following cases: {:?}", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn scope_empty<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.scope_empty;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if header.scope.is_empty() =>
        {
            Err((level, "Scope must not be empty.".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if !header.scope.is_empty() => {
            Err((level, "Scope must be empty.".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn scope_max_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.scope_max_length;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if &header.scope.len() > &value =>
        {
            Err((
                level,
                format!("Scope must not be longer than {} characters.", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if &header.scope.len() <= &value =>
        {
            Err((
                level,
                format!("Scope must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn scope_min_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.scope_min_length;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if &header.scope.len() < &value =>
        {
            Err((
                level,
                format!("Scope must be longer than {} characters.", value),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if &header.scope.len() >= &value =>
        {
            Err((
                level,
                format!("Scope must not be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}
