use crate::core::{search, CoreError};
use crate::utils::cli::print_person;
use crate::utils::db::join_filters;

pub fn run(dept: &Option<String>, email: &Option<String>) -> Result<(), CoreError> {
    let people = search(&join_filters(dept, email))?;
    print_person(people, vec!["created"]);
    Ok(())
}
