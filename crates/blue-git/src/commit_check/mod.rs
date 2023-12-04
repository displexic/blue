use blue_config::git::commit_check::{CommitCheckConfig, Level, RequiredCommitCheckRules};
use colored::Colorize;

pub mod find_case;
mod merge_config;
mod parse_commit;
mod rules;

use parse_commit::Commit;

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
                eprintln!("{} {}", "Warning:".yellow().bold(), message.yellow());
                results.0.push(message);
            }
            Level::Error => {
                eprintln!("{} {}", "Error:".red().bold(), message.red());
                results.1.push(message);
            }
        },
    };

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

    if !results.0.is_empty() {
        eprintln!(
            "\n{} {}\n",
            " WARNING ".on_yellow().bold(),
            "Commit message contains warnings".yellow()
        );
    } else if !results.1.is_empty() {
        eprintln!(
            "\n{} {}\n",
            " ERROR ".on_red().bold(),
            "Commit message failed checks".red()
        );
        std::process::exit(1);
    } else {
        eprintln!(
            "\n{} {}\n",
            " SUCCESS ".on_green().bold(),
            "Commit message passed checks".green()
        );
    }
}
