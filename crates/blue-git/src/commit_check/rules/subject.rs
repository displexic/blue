use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

use crate::commit_check::{find_case::find_case, parse_commit::CommitHeader};

pub fn subject_case<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.subject_case;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !find_case(&header.content, value) =>
        {
            return Err((
                level,
                format!("Header must be in one of the following cases: {:?}", value),
            ));
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if find_case(&header.content, value) =>
        {
            return Err((
                level,
                format!("Header must not be any of the following cases: {:?}", value),
            ));
        }

        _ => Ok(()),
    }
}

pub fn subject_empty<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.subject_empty;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if header.subject.is_empty() =>
        {
            return Err((level, "Header must not be empty.".to_string()));
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if !header.subject.is_empty() =>
        {
            return Err((level, "Header must be empty.".to_string()));
        }
        _ => Ok(()),
    }
}

pub fn subject_full_stop<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.subject_full_stop;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !header.subject.ends_with('.') =>
        {
            return Err((level, "Header must end with a full stop (.)".to_string()));
        }

        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if header.subject.ends_with('.') =>
        {
            return Err((
                level,
                "Header must not end with a full stop (.)".to_string(),
            ));
        }

        _ => Ok(()),
    }
}

pub fn subject_max_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.subject_max_length;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if &header.subject.len() > value =>
        {
            Err((
                level,
                format!(
                    "Header must be less than or equal to {} characters long.",
                    value
                ),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if &header.subject.len() <= &value =>
        {
            Err((
                level,
                format!("Header must be greater than {} characters long.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn subject_min_length<'a>(
    header: &CommitHeader,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.subject_min_length;
    match (&level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if &header.subject.len() < value =>
        {
            Err((
                level,
                format!(
                    "Header must be greater than or equal to {} characters long.",
                    value
                ),
            ))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if &header.subject.len() >= value =>
        {
            Err((
                level,
                format!("Header must be less than {} characters long.", value),
            ))
        }
        _ => Ok(()),
    }
}
