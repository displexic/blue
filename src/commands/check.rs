use crate::config::Config;

use super::Check;

pub fn run(config: &Config, check: &Check) {
    println!("Checking environment: {:?}", check.env);
    println!(
        "Workspace Name: {:?}",
        config.workspace.as_ref().unwrap().name.as_ref().unwrap()
    );
}
