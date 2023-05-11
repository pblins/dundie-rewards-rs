use crate::core::{move_points, CoreError};
use crate::database::models::{Person, User};
use crate::utils::cli::print_person;
use crate::utils::db::join_filters;

pub fn run(person: &Person, user: &User, value: f32, to: &String) -> Result<(), CoreError> {
    let mut query = join_filters(&None, &Some(person.email.clone()));
    let sender = &move_points(-value, &user.username, &query)?[0];

    query = join_filters(&None, &Some(to.to_string()));
    let receiver = &move_points(value, &user.username, &query)?[0];
    println!(
        "Success.. {} points transferred from your account to account of {}.",
        value, receiver.name
    );
    print_person(vec![sender.clone()], vec!["created"]);

    Ok(())
}
