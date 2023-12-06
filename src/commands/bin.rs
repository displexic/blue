use std::env;

use clap::Args;

#[derive(Args, Debug)]
pub struct BinArgs {}

pub fn run() {
    let current_path = env::current_exe().unwrap_or_else(|_| {
        tracing::error!("Failed to get current executable path");
        std::process::exit(1);
    });

    tracing::info!("{}", current_path.display());
}
