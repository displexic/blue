use blue_cli::commands::{bin, bootstrap, check, clean, git, setup, version};
use blue_config::Config;
use blue_log::init_tracing;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Blue")]
#[command(version = "0.0.1")]
#[command(author = "Slekup <slekupvimplyrataqq@protonmail.com>")]
#[command(about = "Blue - Fast and extensible workspace manager")]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Returns the path to the binary
    Bin(bin::BinArgs),
    /// Installs Blue into the user's system
    Bootstrap(bootstrap::BootstrapArgs),
    /// Checks if the workspace meets specified requirements
    Check(check::CheckArgs),
    /// Cleans up temporary files and directories
    Clean(clean::CleanArgs),
    // Git commands
    Git(git::GitArgs),
    // Setup the development environment
    Setup(setup::SetupArgs),
    /// Gets the currently installed version of Blue
    Version(version::VersionArgs),
}

fn require_config(config: &Option<Config>) -> &Config {
    match config {
        Some(config) => config,
        None => {
            tracing::error!("No blue.toml found in current directory");
            std::process::exit(1);
        }
    }
}

fn main() {
    let config_filename = "blue.toml";

    let config_contents: Option<String> = std::fs::read_to_string(config_filename).ok();
    let config: Option<Config> = match config_contents {
        Some(contents) => toml::from_str(&contents).unwrap_or_else(|err| {
            tracing::error!("{} is not valid toml", config_filename);
            tracing::error!("{}", err);
            std::process::exit(1);
        }),
        None => None,
    };

    // Initialize the logger
    init_tracing(&config.as_ref().unwrap().debug);

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
        Some(Commands::Clean(command)) => {
            clean::run(command, require_config(&config));
        }
        Some(Commands::Git(git_command)) => match &git_command.commands {
            git::GitCommands::CommitCheck(command) => {
                git::commit_check::run(&command, require_config(&config));
            }
        },
        Some(Commands::Setup(_command)) => {
            setup::run();
        }
        Some(Commands::Version(_command)) => {
            version::run();
        }
        None => {
            tracing::error!("No command specified. Use --help for more information.");
        }
    }
}
