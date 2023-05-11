pub mod auth;
pub mod cli;
pub mod core;
pub mod database;
pub mod security;
pub mod serializers;
pub mod utils;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();
    match cli::match_command(&cli) {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error)
        }
    }
}
