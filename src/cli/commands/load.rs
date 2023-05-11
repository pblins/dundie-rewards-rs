use crate::core::{load, CoreError};
use crate::utils::cli::print_person;

pub fn run(filepath: &String) -> Result<(), CoreError> {
    let people = load(filepath.to_string())?;
    print_person(people, vec!["balance", "value"]);

    Ok(())
}
