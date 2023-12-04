use blue_config::git::commit_check::{Level, RequiredCommitCheckRules, Rule};

pub fn footer_leading_blank<'a>(
    footer: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.footer_leading_blank;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always)
            if footer.len() > 0 && !footer[footer.len() - 1].is_empty() =>
        {
            Err((level, "Footer must have a leading blank line.".to_string()))
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never)
            if footer.len() > 0 && footer[footer.len() - 1].is_empty() =>
        {
            Err((
                level,
                "Footer must not have a leading blank line.".to_string(),
            ))
        }
        _ => Ok(()),
    }
}

pub fn footer_empty<'a>(
    footer: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule) = &rules.footer_empty;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if footer.len() == 0 || (footer.len() > 0 && !footer.iter().any(|s| !s.is_empty())) {
                Err((level, "Footer must not be empty.".to_string()))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if footer.len() > 0 => {
            Err((level, "Footer must be empty.".to_string()))
        }
        _ => Ok(()),
    }
}

pub fn footer_max_line_length<'a>(
    footer: &Vec<String>,
    rules: &'a RequiredCommitCheckRules,
) -> Result<(), (&'a Level, String)> {
    let (level, rule, value) = &rules.body_max_line_length;
    match (level, rule) {
        (Level::Warning, Rule::Always) | (Level::Error, Rule::Always) => {
            if &footer.len() > &value {
                Err((
                    level,
                    format!("Footer lines must not be longer than {} characters.", value),
                ))
            } else {
                Ok(())
            }
        }
        (Level::Warning, Rule::Never) | (Level::Error, Rule::Never) if &footer.len() <= &value => {
            Err((
                level,
                format!("Footer lines must be longer than {} characters.", value),
            ))
        }
        _ => Ok(()),
    }
}
