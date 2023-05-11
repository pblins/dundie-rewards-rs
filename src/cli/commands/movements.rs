use crate::core::{get_statement, CoreError};
use crate::database::models::User;
use crate::utils::cli::print_statement;

pub fn run(user: &User) -> Result<(), CoreError> {
    let (balance, movements) = get_statement(user.person_id)?;
    print_statement(balance, movements);
    Ok(())
}
