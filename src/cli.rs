pub mod commands;

use std::env;
use std::io::{self, Write};

use clap::{Parser, Subcommand};

use crate::auth::{authenticate_user, AuthenticationError};
use crate::core::CoreError;
use crate::database::models::{Person, User};

pub fn app() {
    let cli = Cli::parse();
    match match_command(&cli) {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error)
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    Core(CoreError),
    Authentication(AuthenticationError),
}

impl From<CoreError> for CliError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

impl From<AuthenticationError> for CliError {
    fn from(value: AuthenticationError) -> Self {
        Self::Authentication(value)
    }
}

#[derive(Parser)]
#[command(name = "dundie-rewards")]
#[command(author = "Paulo Branco <paulorobertobranco@gmail.com>")]
#[command(about = "Dunder Mifflin Rewards System.
 This cli application controls Dunder Mifflin rewards.
  - admins can load information tot he people database and assign points.
  - users can view reports and transfer points.", long_about = None)]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Loads the file to the database.", long_about = None)]
    Load {
        filepath: String,
    },
    #[command(about = "Shows information about user or dept.", long_about = None)]
    Show {
        #[arg(short, long)]
        dept: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    #[command(about = "Add points to the user or dept.", long_about = None)]
    Add {
        value: f32,
        #[arg(short, long)]
        dept: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    #[command(about = "Remove points to the user or dept.", long_about = None)]
    Remove {
        value: f32,
        #[arg(short, long)]
        dept: Option<String>,
        #[arg(short, long)]
        email: Option<String>,
    },
    Transfer {
        #[arg(short, long)]
        value: f32,
        #[arg(short, long)]
        to: String,
    },
    Movements,
}

impl Authenticated for Commands {
    fn authenticate(
        &self,
        requires_superuser: bool,
    ) -> Result<(Person, User), AuthenticationError> {
        let mut username = String::new();
        let password;

        match env::var("DUNDIE_USER") {
            Ok(user) => username = user,
            Err(_) => {
                print!("username: ",);
                let _ = io::stdout().flush();
                io::stdin().read_line(&mut username).unwrap();
                username = username.trim().to_string();
            }
        }
        match env::var("DUNDIE_PWD") {
            Ok(pass) => password = pass,
            Err(_) => {
                password = rpassword::prompt_password("password: ").unwrap();
            }
        }

        authenticate_user(&username, &password, requires_superuser)
    }
}

pub fn match_command(cli: &Cli) -> Result<(), CliError> {
    match &cli.command {
        Commands::Load { filepath } => {
            let _ = &cli.command.authenticate(true)?;
            commands::load::run(filepath)?;
            return Ok(());
        }
        Commands::Show { dept, email } => {
            let (person, user) = &cli.command.authenticate(false)?;

            if user.superuser {
                commands::show::run(dept, email)?;
            } else {
                commands::show::run(&None, &Some(person.email.clone()))?;
            }

            return Ok(());
        }
        Commands::Add { value, dept, email } => {
            let (_, user) = &cli.command.authenticate(true)?;
            commands::add::run(user, *value, dept, email)?;
            return Ok(());
        }
        Commands::Remove { value, dept, email } => {
            let (_, user) = &cli.command.authenticate(true)?;
            commands::add::run(user, -(*value), dept, email)?;
            return Ok(());
        }
        Commands::Transfer { value, to } => {
            let (sender, user) = &cli.command.authenticate(false)?;
            commands::transfer::run(sender, user, *value, to)?;
            return Ok(());
        }
        Commands::Movements => {
            let (_, user) = &cli.command.authenticate(false)?;
            commands::movements::run(user)?;
            return Ok(());
        }
    }
}

pub trait Authenticated {
    fn authenticate(&self, requires_superuser: bool)
        -> Result<(Person, User), AuthenticationError>;
}
