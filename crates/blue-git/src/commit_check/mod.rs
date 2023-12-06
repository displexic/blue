use blue_config::git::commit_check::{CommitCheckConfig, Level, RequiredCommitCheckRules};

pub mod find_case;
mod merge_config;
mod parse_commit;
mod presets;
mod rules;

use parse_commit::Commit;

use crate::git_hooks::{GitHook, GitHookType};

/// 1. Parse the commit message into the Commit struct.
/// 2. Merge the preset (if defined) with the user defined config rules.
/// 3. Run the rules against the commit message.
/// 4. Handle the results. Print if warning or error, and add to the results vector. Exit with error if error.
pub fn run(commit_message: String, config: &CommitCheckConfig) {
    let mut results: (Vec<String>, Vec<String>) = (vec![], vec![]);
    let commit = Commit::parse_message(&mut commit_message.clone());

    let config: RequiredCommitCheckRules = merge_config::merge_preset_with_config(
        config.rules.as_ref().unwrap().clone(),
        &config.preset.as_ref().unwrap(),
    );

    let mut handle_rule_check = |result: Result<(), (&Level, String)>| match result {
        Ok(_) => (),
        Err((level, message)) => match level {
            Level::Disabled => (),
            Level::Warning => {
                tracing::warn!("{}", message);
                results.0.push(message);
            }
            Level::Error => {
                tracing::error!("{}", message);
                results.1.push(message);
            }
        },
    };

    handle_rule_check(rules::body::body_full_stop(&commit.body, &config));
    handle_rule_check(rules::body::body_leading_blank(&commit.body, &config));
    handle_rule_check(rules::body::body_empty(&commit.body, &config));
    handle_rule_check(rules::body::body_max_length(&commit.body, &config));
    handle_rule_check(rules::body::body_min_length(&commit.body, &config));
    handle_rule_check(rules::body::body_max_line_length(&commit.body, &config));
    handle_rule_check(rules::body::body_min_line_length(&commit.body, &config));
    handle_rule_check(rules::body::body_max_lines(&commit.body, &config));
    handle_rule_check(rules::body::body_case(&commit.body, &config));

    handle_rule_check(rules::commit_type::type_enum(&commit.header, &config));
    handle_rule_check(rules::commit_type::type_case(&commit.header, &config));
    handle_rule_check(rules::commit_type::type_empty(&commit.header, &config));
    handle_rule_check(rules::commit_type::type_max_length(&commit.header, &config));
    handle_rule_check(rules::commit_type::type_min_length(&commit.header, &config));

    handle_rule_check(rules::footer::footer_leading_blank(&commit.footer, &config));
    handle_rule_check(rules::footer::footer_empty(&commit.footer, &config));
    handle_rule_check(rules::footer::footer_max_line_length(
        &commit.footer,
        &config,
    ));

    handle_rule_check(rules::header::header_case(&commit.header, &config));
    handle_rule_check(rules::header::header_full_stop(&commit.header, &config));
    handle_rule_check(rules::header::header_max_length(&commit.header, &config));
    handle_rule_check(rules::header::header_min_length(&commit.header, &config));

    handle_rule_check(rules::references_empty(&commit_message, &config));

    handle_rule_check(rules::scope::scope_enum(&commit.header, &config));
    handle_rule_check(rules::scope::scope_case(&commit.header, &config));
    handle_rule_check(rules::scope::scope_empty(&commit.header, &config));
    handle_rule_check(rules::scope::scope_max_length(&commit.header, &config));
    handle_rule_check(rules::scope::scope_min_length(&commit.header, &config));

    handle_rule_check(rules::subject::subject_case(&commit.header, &config));
    handle_rule_check(rules::subject::subject_empty(&commit.header, &config));
    handle_rule_check(rules::subject::subject_full_stop(&commit.header, &config));
    handle_rule_check(rules::subject::subject_max_length(&commit.header, &config));
    handle_rule_check(rules::subject::subject_min_length(&commit.header, &config));

    handle_rule_check(rules::signed_off_by(&commit_message, &config));
    handle_rule_check(rules::trailer(&mut commit_message.clone(), &config));

    println!("");

    if !results.0.is_empty() {
        tracing::warn!("Commit message contains warnings")
    }

    if !results.1.is_empty() {
        tracing::error!("Commit message contains errors");
        std::process::exit(1);
    }

    if results.0.is_empty() && results.0.is_empty() {
        tracing::info!("Commit message passed checks");
    }

    println!("");
}

pub fn init_git_hooks() {
    let commit_msg = GitHook::new(GitHookType::CommitMsg, None);
    commit_msg.create("#!/usr/bin/env sh\n\nmessage=$(cat $1)\nexec blue git commit-check --message \"${message}\"");
}
