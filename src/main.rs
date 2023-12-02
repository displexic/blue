use blue_cli::commands::bin;
use blue_cli::commands::bootstrap;
use blue_cli::commands::check;
use blue_cli::commands::version;
use blue_cli::config::Config;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// Returns the path to the binary
    Bin(bin::Bin),
    /// Installs Blue into the user's system
    Bootstrap(bootstrap::Bootstrap),
    /// Checks if the workspace meets specified requirements
    Check(check::Check),
    /// Gets the currently installed version of Blue
    Version(version::Version),
}

#[derive(Parser, Debug)]
#[command(name = "blue", version = "0.0.1", author = "Slekup")]
#[command(about = "blue - Fast and extensible workspace manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn require_config(config: &Option<Config>) -> &Config {
    match config {
        Some(config) => config,
        None => {
            println!("No blue.toml found in current directory");
            std::process::exit(1);
        }
    }
}

fn main() {
    let config_filename = "blue.toml";

    let config_contents: Option<String> = std::fs::read_to_string(config_filename).ok();
    let config: Option<Config> = match config_contents {
        Some(contents) => toml::from_str(&contents)
            .unwrap_or_else(|_| panic!("{} is not valid toml", config_filename)),
        None => None,
    };

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Bin(_command)) => {
            bin::run();
        }
        Some(Commands::Bootstrap(_command)) => {
            bootstrap::run();
        }
        Some(Commands::Check(command)) => {
            check::run(command, require_config(&config));
        }
        Some(Commands::Version(_command)) => {
            version::run();
        }
        None => {
            println!("No command specified");
        }
    }
}
