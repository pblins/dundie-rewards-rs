use crate::core::{move_points, search, CoreError};
use crate::database::models::User;
use crate::utils::cli::print_person;
use crate::utils::db::join_filters;

pub fn run(
    user: &User,
    value: f32,
    dept: &Option<String>,
    email: &Option<String>,
) -> Result<(), CoreError> {
    let query = join_filters(dept, email);
    move_points(value, &user.username, &query)?;

    let people = search(&query)?;
    print_person(people, vec!["created"]);

    Ok(())
}
