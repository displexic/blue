//! The configuration is a bit different for presets, since we are not restricted to TOML's syntax.
//! (TOML does not support tuples or multi-type arrays)
//! Instead of each configuration being an object with the properties `level` `rule` and `value`,
//! the configuration is a tuple of `(level, rule, value)`.

use blue_config::git::commit_check::{Case, Level, RequiredCommitCheckRules, Rule};

pub fn default() -> RequiredCommitCheckRules {
    RequiredCommitCheckRules {
        body_full_stop: (Level::Disabled, Rule::Never),
        body_leading_blank: (Level::Warning, Rule::Always),
        body_empty: (Level::Disabled, Rule::Never),
        body_max_length: (Level::Error, Rule::Always, 10_000),
        body_min_length: (Level::Disabled, Rule::Always, 0),
        body_max_line_length: (Level::Error, Rule::Always, 100),
        body_min_line_length: (Level::Disabled, Rule::Always, 0),
        body_max_lines: (Level::Disabled, Rule::Always, 10_000),
        body_case: (Level::Error, Rule::Always, vec![Case::Lower]),

        footer_leading_blank: (Level::Warning, Rule::Always),
        footer_empty: (Level::Disabled, Rule::Never),
        footer_max_length: (Level::Disabled, Rule::Always, 10_000),
        footer_max_line_length: (Level::Error, Rule::Always, 100),

        header_case: (Level::Error, Rule::Always, vec![Case::Lower]),
        header_full_stop: (Level::Error, Rule::Never),
        header_max_length: (Level::Error, Rule::Always, 100),
        header_min_length: (Level::Error, Rule::Always, 10),

        references_empty: (Level::Disabled, Rule::Never),

        scope_enum: (Level::Disabled, Rule::Always, vec![]),
        scope_case: (Level::Error, Rule::Always, vec![Case::Lower]),
        scope_empty: (Level::Disabled, Rule::Never),
        scope_max_length: (Level::Error, Rule::Always, 100),
        scope_min_length: (Level::Error, Rule::Always, 2),

        subject_case: (
            Level::Error,
            Rule::Never,
            vec![Case::Sentence, Case::Start, Case::Pascal, Case::Upper],
        ),
        subject_empty: (Level::Error, Rule::Never),
        subject_full_stop: (Level::Error, Rule::Never),
        subject_max_length: (Level::Error, Rule::Always, 100),
        subject_min_length: (Level::Error, Rule::Always, 10),
        subject_exclamation: (Level::Disabled, Rule::Never),

        type_enum: (
            Level::Error,
            Rule::Always,
            vec![
                "build".to_string(),
                "chore".to_string(),
                "ci".to_string(),
                "docs".to_string(),
                "feat".to_string(),
                "fix".to_string(),
                "perf".to_string(),
                "refactor".to_string(),
                "revert".to_string(),
                "style".to_string(),
                "test".to_string(),
            ],
        ),
        type_case: (Level::Error, Rule::Always, vec![Case::Lower]),
        type_empty: (Level::Error, Rule::Never),
        type_max_length: (Level::Error, Rule::Always, 100),
        type_min_length: (Level::Error, Rule::Always, 2),

        signed_off_by: (Level::Disabled, Rule::Always, "Signed-off-by:".to_string()),
        trailer: (Level::Disabled, Rule::Always, "Signed-off-by:".to_string()),
    }
}
