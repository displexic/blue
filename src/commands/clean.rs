use clap::Args;
use glob::glob;
use std::fs;

use blue_config::Config;

#[derive(Args, Debug)]

pub struct CleanArgs {
    /// Files and directories to clean
    files: Option<Vec<String>>,
}

pub fn run(command: &CleanArgs, config: &Config) {
    let mut files: Vec<String> = vec![];

    if let Some(config_clean_files) = &config.workspace.as_ref().unwrap().clean_files {
        for file in config_clean_files.iter() {
            files.push(file.to_string());
        }
    }

    if let Some(command_files) = &command.files {
        for file in command_files.iter() {
            files.push(file.to_string());
        }
    }

    if files.iter().len() == 0 {
        tracing::error!("No files or directories defined in config");
        std::process::exit(1);
    }

    for file in files.iter() {
        for entry in glob(file).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        fs::remove_file(path).expect("Failed to remove file");
                    } else {
                        fs::remove_dir_all(path).expect("Failed to remove directory");
                    }
                }
                Err(e) => tracing::error!("{:?}", e),
            }
        }
    }
}
