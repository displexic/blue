//! Merge the configuration rules into the preset rules.
//! This should not be run if the number of rules in the configuration is 0.
//! This is also only run when a preset is provided.

use blue_config::git::commit_check::{
    CaseRule, CommitCheckPresets, CommitCheckRules, DefaultRule, NumberRule, RequiredCaseRule,
    RequiredCommitCheckRules, RequiredDefaultRule, RequiredNumberRule, RequiredStringListRule,
    RequiredStringRule, StringListRule, StringRule,
};

use super::presets;

fn get_preset(preset: &CommitCheckPresets) -> RequiredCommitCheckRules {
    match preset {
        CommitCheckPresets::Default => presets::default(),
    }
}

fn merge_default(old: &mut RequiredDefaultRule, new: DefaultRule) {
    new.level.map(|new_level| old.0 = new_level);
    new.rule.map(|new_rule| old.1 = new_rule);
}

fn merge_number(old: &mut RequiredNumberRule, new: NumberRule) {
    new.level.map(|new_level| old.0 = new_level);
    new.rule.map(|new_rule| old.1 = new_rule);
    new.value.map(|new_value| old.2 = new_value);
}

fn merge_case(old: &mut RequiredCaseRule, new: CaseRule) {
    new.level.map(|new_level| old.0 = new_level);
    new.rule.map(|new_rule| old.1 = new_rule);
    new.value.map(|new_value| old.2 = new_value);
}

fn merge_string(old: &mut RequiredStringRule, new: StringRule) {
    new.level.map(|new_level| old.0 = new_level);
    new.rule.map(|new_rule| old.1 = new_rule);
    new.value.map(|new_value| old.2 = new_value);
}

fn merge_string_list(old: &mut RequiredStringListRule, new: StringListRule) {
    new.level.map(|new_level| old.0 = new_level);
    new.rule.map(|new_rule| old.1 = new_rule);
    new.value.map(|new_value| old.2 = new_value);
}

pub fn merge_preset_with_config(
    config_rules: CommitCheckRules,
    preset: &CommitCheckPresets,
) -> RequiredCommitCheckRules {
    let mut merged_config: RequiredCommitCheckRules = get_preset(&preset).clone();

    config_rules
        .body_full_stop
        .map(|body_full_stop| merge_default(&mut merged_config.body_full_stop, body_full_stop));
    config_rules.body_leading_blank.map(|body_leading_blank| {
        merge_default(&mut merged_config.body_leading_blank, body_leading_blank)
    });
    config_rules
        .body_empty
        .map(|body_empty| merge_default(&mut merged_config.body_empty, body_empty));
    config_rules
        .body_max_length
        .map(|body_max_length| merge_number(&mut merged_config.body_max_length, body_max_length));
    config_rules
        .body_min_length
        .map(|body_min_length| merge_number(&mut merged_config.body_min_length, body_min_length));
    config_rules
        .body_max_line_length
        .map(|body_max_line_length| {
            merge_number(
                &mut merged_config.body_max_line_length,
                body_max_line_length,
            )
        });
    config_rules
        .body_min_line_length
        .map(|body_min_line_length| {
            merge_number(
                &mut merged_config.body_min_line_length,
                body_min_line_length,
            )
        });
    config_rules
        .body_max_lines
        .map(|body_max_lines| merge_number(&mut merged_config.body_max_lines, body_max_lines));
    config_rules
        .body_case
        .map(|body_case| merge_case(&mut merged_config.body_case, body_case));

    config_rules
        .footer_leading_blank
        .map(|footer_leading_blank| {
            merge_default(
                &mut merged_config.footer_leading_blank,
                footer_leading_blank,
            )
        });
    config_rules
        .footer_empty
        .map(|footer_empty| merge_default(&mut merged_config.footer_empty, footer_empty));
    config_rules.footer_max_length.map(|footer_max_length| {
        merge_number(&mut merged_config.footer_max_length, footer_max_length)
    });
    config_rules
        .footer_max_line_length
        .map(|footer_max_line_length| {
            merge_number(
                &mut merged_config.footer_max_line_length,
                footer_max_line_length,
            )
        });

    config_rules
        .header_case
        .map(|header_case| merge_case(&mut merged_config.header_case, header_case));
    config_rules.header_full_stop.map(|header_full_stop| {
        merge_default(&mut merged_config.header_full_stop, header_full_stop)
    });
    config_rules.header_max_length.map(|header_max_length| {
        merge_number(&mut merged_config.header_max_length, header_max_length)
    });
    config_rules.header_min_length.map(|header_min_length| {
        merge_number(&mut merged_config.header_min_length, header_min_length)
    });

    config_rules.references_empty.map(|references_empty| {
        merge_default(&mut merged_config.references_empty, references_empty)
    });

    config_rules
        .scope_enum
        .map(|scope_enum| merge_string_list(&mut merged_config.scope_enum, scope_enum));
    config_rules
        .scope_case
        .map(|scope_case| merge_case(&mut merged_config.scope_case, scope_case));
    config_rules
        .scope_empty
        .map(|scope_empty| merge_default(&mut merged_config.scope_empty, scope_empty));
    config_rules.scope_max_length.map(|scope_max_length| {
        merge_number(&mut merged_config.scope_max_length, scope_max_length)
    });
    config_rules.scope_min_length.map(|scope_min_length| {
        merge_number(&mut merged_config.scope_min_length, scope_min_length)
    });

    config_rules
        .subject_case
        .map(|subject_case| merge_case(&mut merged_config.subject_case, subject_case));
    config_rules
        .subject_empty
        .map(|subject_empty| merge_default(&mut merged_config.subject_empty, subject_empty));
    config_rules.subject_full_stop.map(|subject_full_stop| {
        merge_default(&mut merged_config.subject_full_stop, subject_full_stop)
    });
    config_rules.subject_max_length.map(|subject_max_length| {
        merge_number(&mut merged_config.subject_max_length, subject_max_length)
    });
    config_rules.subject_min_length.map(|subject_min_length| {
        merge_number(&mut merged_config.subject_min_length, subject_min_length)
    });

    config_rules
        .type_enum
        .map(|type_enum| merge_string_list(&mut merged_config.type_enum, type_enum));
    config_rules
        .type_case
        .map(|type_case| merge_case(&mut merged_config.type_case, type_case));
    config_rules
        .type_empty
        .map(|type_empty| merge_default(&mut merged_config.type_empty, type_empty));
    config_rules
        .type_max_length
        .map(|type_max_length| merge_number(&mut merged_config.type_max_length, type_max_length));
    config_rules
        .type_min_length
        .map(|type_min_length| merge_number(&mut merged_config.type_min_length, type_min_length));

    config_rules
        .signed_off_by
        .map(|signed_off_by| merge_string(&mut merged_config.signed_off_by, signed_off_by));
    config_rules
        .trailer
        .map(|trailer| merge_string(&mut merged_config.trailer, trailer));

    merged_config
}
