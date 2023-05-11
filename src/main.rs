pub mod auth;
pub mod cli;
pub mod core;
pub mod database;
pub mod security;
pub mod serializers;
pub mod utils;

fn main() {
    cli::app();
}
