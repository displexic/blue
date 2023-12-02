use std::env;

use clap::Args;

#[derive(Args, Debug)]
pub struct Version {}

pub fn run() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("{}", VERSION);
}
