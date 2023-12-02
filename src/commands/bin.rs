use std::env;

use clap::Args;

#[derive(Args, Debug)]
pub struct Bin {}

pub fn run() {
    let current_path = env::current_exe().unwrap_or_else(|_| {
        eprintln!("Failed to get current executable path");
        std::process::exit(1);
    });

    println!("{}", current_path.display());
}
