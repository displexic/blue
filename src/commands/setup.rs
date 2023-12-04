use clap::Args;

#[derive(Args, Debug)]
pub struct SetupArgs {}

pub fn run() {
    blue_git::setup();
}
