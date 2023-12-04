use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

use crate::commit_check::find_case::find_case;

pub fn body_full_stop<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.body_full_stop;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !body[body.len() - 1].ends_with(".") =>
        {
            Err((level, "Body must end with a full stop (\".\").".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if body[body.len() - 1].ends_with(".") =>
        {
            Err((
                level,
                "Body must not end with a full stop (\".\").".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_leading_blank<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.body_leading_blank;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if body.len() > 0 && !body[body.len() - 1].is_empty() =>
        {
            Err((level, "Body must have a leading blank line.".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if body.len() > 0 && body[body.len() - 1].is_empty() =>
        {
            Err((
                level,
                "Body must not have a leading blank line.".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_empty<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.body_empty;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if body.len() == 0 || (body.len() > 0 && !body.iter().any(|s| !s.is_empty())) {
                Err((level, "Body must not be empty.".to_string()))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if body.len() > 0 => {
            Err((level, "Body must be empty.".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn body_max_length<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_max_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &body.len() > &value {
                Err((
                    level,
                    format!("Body must not be longer than {} characters.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &body.len() <= &value => {
            Err((
                level,
                format!("Body must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_min_length<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_min_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &body.len() < &value {
                Err((
                    level,
                    format!("Body must not be longer than {} characters.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &body.len() >= &value => {
            Err((
                level,
                format!("Body must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_max_line_length<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_max_line_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &body.len() > &value {
                Err((
                    level,
                    format!("Body lines must not be longer than {} characters.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &body.len() <= &value => {
            Err((
                level,
                format!("Body lines must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_min_line_length<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_min_line_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &body.len() < &value {
                Err((
                    level,
                    format!("Body lines must not be longer than {} characters.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &body.len() >= &value => {
            Err((
                level,
                format!("Body lines must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}

pub fn body_max_lines<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_max_lines;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &body.len() > &value {
                Err((
                    level,
                    format!("Body must not be longer than {} lines.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &body.len() <= &value => {
            Err((level, format!("Body must be longer than {} lines.", value)))
        }
        _ => Ok(()),
    }
}

pub fn body_case<'a>(
    body: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_case;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if !find_case(&body.join(" "), value) =>
        {
            return Err((
                level,
                format!("Body must be in one of the following cases: {:?}", value),
            ));
        }

        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if find_case(&body.join(" "), value) =>
        {
            return Err((
                level,
                format!("Body must not be any of the following cases: {:?}", value),
            ));
        }
        _ => Ok(()),
    }
}
