use blue::commands::{self, Check};
use blue::config::Config;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Commands {
    /// Checks if the workspace meets specified requirements
    Check(Check),
}

#[derive(Parser, Debug)]
#[command(name = "blue", version = "0.0.1", author = "Slekup")]
#[command(about = "blue - Fast and extensible workspace manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let config_filename = "blue.toml";
    let config_contents = std::fs::read_to_string(config_filename)
        .expect(format!("{} not found", config_filename).as_str());
    let config: Config = toml::from_str(&config_contents)
        .expect(format!("{} is not valid toml", config_filename).as_str());

    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Check(check)) => {
            commands::check::run(&config, check);
        }
        None => {
            println!("No command specified");
        }
    }
}
