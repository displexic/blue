use clap::Args;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

#[derive(Args, Debug)]
pub struct Setup {}

pub fn run() {
    println!("Setting up blue...");

    let current_path = env::current_exe().expect("Failed to get current executable path");
    let binary_name = current_path.file_name().expect("Failed to get binary name");

    // Get the recommended directory based on the current operating system
    let recommended_dir: PathBuf = match env::consts::OS {
        "windows" => {
            let home_dir = match home::home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Impossible to get your home dir!");
                    std::process::exit(1);
                }
            };

            let target_dir_str = format!("{}\\.blue\\bin", home_dir.to_str().unwrap());
            let target_dir = PathBuf::from(target_dir_str);

            println!("Target dir 555: {:?}", target_dir.to_str().unwrap());

            // Create the directory if it doesn't exist
            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).unwrap_or_else(|_| {
                    panic!(
                        "Failed to create directory: {}",
                        &target_dir.to_str().unwrap()
                    )
                });
            }

            target_dir
        }
        "linux" => {
            let home_dir = match home::home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Impossible to get your home dir!");
                    std::process::exit(1);
                }
            };

            let target_dir_str = format!("{}/.blue/bin", home_dir.to_str().unwrap());
            let target_dir = PathBuf::from(&target_dir_str);

            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).unwrap_or_else(|_| {
                    panic!(
                        "Failed to create directory: {}",
                        &target_dir.to_str().unwrap()
                    )
                });
            }

            target_dir
        }
        "macos" => {
            let target_dir = PathBuf::from("/usr/local/bin/.blue/bin");
            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).unwrap_or_else(|_| {
                    panic!(
                        "Failed to create directory: {}",
                        &target_dir.to_str().unwrap()
                    )
                });
            }
            target_dir
        }
        _ => {
            eprintln!("Unsupported operating system");
            return;
        }
    };

    let new_path = recommended_dir.join(binary_name);

    if let Err(err) = fs::copy(&current_path, &new_path) {
        eprintln!(
            "Failed to move the binary to {}: {}",
            new_path.display(),
            err
        );
        std::process::exit(1);
    }

    println!("Adding {:?} to PATH", &recommended_dir.to_str().unwrap());

    // Set the new PATH variable for the current process and child processes based on the OS
    match env::consts::OS {
        "windows" => {
            let cmd = &format!(
                "setx PATH \"{};$Env:PATH\"",
                &recommended_dir.to_str().unwrap()
            );
            println!("Running command: {}", cmd);
            let result = Command::new("powershell").arg(cmd).status();

            if let Err(err) = result {
                eprintln!("Failed to set PATH variable: {}", err);
            } else {
                // Success message
                println!("Please restart your terminal to use blue");
            }
        }
        "linux" | "macos" => {
            let cmd = &format!("export PATH=$PATH:{}", &recommended_dir.to_str().unwrap());

            // add export command to ~/.bashrc
            let home_dir = match home::home_dir() {
                Some(path) => path,
                None => {
                    eprintln!("Impossible to get your home dir!");
                    std::process::exit(1);
                }
            };

            let required_line = cmd.as_str();

            let bash_file = if env::consts::OS == "linux" {
                ".bashrc"
            } else {
                ".bash_profile"
            };

            let file_path = format!("{}/{}", home_dir.to_str().unwrap(), bash_file);
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file_path)
                .expect("Failed to open file");

            let mut found = false;
            let reader = io::BufReader::new(&file);

            for line in reader.lines().flatten() {
                if line.contains(required_line) {
                    found = true;
                    break;
                }
            }

            if !found {
                let result = writeln!(&file, "{}", &required_line);

                if let Err(err) = result {
                    eprintln!("Failed to write to file: {}", err);
                } else {
                    // Success message
                    println!("Please restart your terminal to use blue or run the following command: source ~/.bashrc");
                }

                println!("{} appended to the file!", &required_line);
            } else {
                println!("{} already exists in the file.", &required_line);
            }
        }
        _ => {
            eprintln!("Unsupported operating system");
            std::process::exit(1);
        }
    };
}
