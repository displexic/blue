use std::env;

use clap::Args;

#[derive(Args, Debug)]
pub struct VersionArgs {}

pub fn run() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    tracing::info!("{}", VERSION);
}
