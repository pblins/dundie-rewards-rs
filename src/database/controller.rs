use std::collections::HashMap;

use diesel::prelude::*;
use serde_valid::Validate;
use slugify::slugify;

use crate::database::models::{
    Balance, Movement, NewBalance, NewMovement, NewPerson, NewUser, Person, User,
};
use crate::database::schema::balance::dsl as balance;
use crate::database::schema::balance::table as balance_table;
use crate::database::schema::movement::table as movement_table;
use crate::database::schema::person::dsl as person;
use crate::database::schema::person::table as person_table;
use crate::database::schema::user::dsl as user;
use crate::database::schema::user::table as user_table;
use crate::utils::user::generate_simple_password;

#[derive(Debug)]
pub enum ControllerError {
    Database(diesel::result::Error),
    Validation(serde_valid::validation::Errors),
    InsufficientBalance,
}

impl From<diesel::result::Error> for ControllerError {
    fn from(value: diesel::result::Error) -> Self {
        Self::Database(value)
    }
}

impl From<serde_valid::validation::Errors> for ControllerError {
    fn from(value: serde_valid::validation::Errors) -> Self {
        Self::Validation(value)
    }
}

pub fn query_person_by_id(
    connection: &mut SqliteConnection,
    person_id: i32,
) -> Result<Person, ControllerError> {
    Ok(person_table
        .filter(person::id.eq(person_id))
        .first::<Person>(connection)?)
}

pub fn add_person(
    connection: &mut SqliteConnection,
    new_person: NewPerson,
) -> Result<(Person, bool), ControllerError> {
    new_person.validate()?;

    match person_exists(connection, &new_person.email) {
        Some(existing_person) => {
            let updated_person = diesel::update(&existing_person)
                .set((
                    person::name.eq(&new_person.name),
                    person::currency.eq(&new_person.currency),
                    person::dept.eq(&new_person.dept),
                    person::role.eq(&new_person.role),
                ))
                .get_result::<Person>(connection)?;

            Ok((updated_person, false))
        }
        None => {
            let added_person = diesel::insert_into(person_table)
                .values(&new_person)
                .get_result::<Person>(connection)?;

            set_initial_password(connection, &added_person)?;
            set_initial_balance(connection, &added_person)?;

            Ok((added_person, true))
        }
    }
}

pub fn query_person(
    connection: &mut SqliteConnection,
    query: &HashMap<String, String>,
) -> Result<Vec<Person>, ControllerError> {
    let result;

    if query.contains_key("email") && query.contains_key("dept") {
        let filter = person::email
            .eq(&query["email"])
            .and(person::dept.eq(&query["dept"]));
        result = person::person.filter(filter).load::<Person>(connection);
    } else if query.contains_key("email") {
        let filter = person::email.eq(&query["email"]);
        result = person::person.filter(filter).load::<Person>(connection);
    } else if query.contains_key("dept") {
        let filter = person::dept.eq(&query["dept"]);
        result = person::person.filter(filter).load::<Person>(connection);
    } else {
        result = person::person.load::<Person>(connection)
    }

    Ok(result?)
}

pub fn query_balance_by_person(
    connection: &mut SqliteConnection,
    person: &Person,
) -> Result<Balance, ControllerError> {
    Ok(Balance::belonging_to(&person)
        .select(Balance::as_select())
        .first(connection)?)
}

fn person_exists(connection: &mut SqliteConnection, search_email: &String) -> Option<Person> {
    let person_exists_result = person_table
        .filter(person::email.eq(search_email))
        .first::<Person>(connection);

    match person_exists_result {
        Ok(existing_person) => Some(existing_person),
        Err(_) => None,
    }
}

pub fn user_exists(connection: &mut SqliteConnection, search_username: &String) -> Option<User> {
    let user_exists_result = user_table
        .filter(user::username.eq(search_username))
        .first::<User>(connection);

    match user_exists_result {
        Ok(existing_user) => Some(existing_user),
        Err(_) => None,
    }
}

fn set_initial_balance(
    connection: &mut SqliteConnection,
    person: &Person,
) -> Result<Balance, ControllerError> {
    let value: f32;

    if person.role == "Manager" {
        value = 100.0;
    } else {
        value = 500.0;
    }

    add_movement(connection, person, value, None)
}

fn set_initial_password(
    connection: &mut SqliteConnection,
    person: &Person,
) -> Result<User, ControllerError> {
    let superuser;

    if person.dept.to_lowercase() == "management" {
        superuser = true;
    } else {
        superuser = false;
    }

    let new_user = diesel::insert_into(user_table)
        .values(
            &(NewUser {
                person_id: person.id,
                password: generate_simple_password(8_usize),
                superuser: superuser,
                username: slugify!(&person.name),
            }),
        )
        .get_result::<User>(connection)?;

    Ok(new_user)
}

pub fn add_movement(
    connection: &mut SqliteConnection,
    person: &Person,
    value: f32,
    actor: Option<String>,
) -> Result<Balance, ControllerError> {
    let actor_str = actor.unwrap_or("system".to_string());

    match query_balance_by_person(connection, &person) {
        Ok(existing_balance) => {
            if existing_balance.value + value < 0.0 {
                return Err(ControllerError::InsufficientBalance);
            }
        }
        Err(_) => (),
    }

    diesel::insert_into(movement_table)
        .values(
            &(NewMovement {
                person_id: person.id,
                value: value,
                actor: actor_str,
            }),
        )
        .get_result::<Movement>(connection)?;

    let person_movements = Movement::belonging_to(&person)
        .select(Movement::as_select())
        .load(connection)?;

    let mut total: f32 = 0.0;
    person_movements.iter().for_each(|mov| total += mov.value);

    match query_balance_by_person(connection, &person) {
        Ok(existing_balance) => {
            let update_balance = diesel::update(&existing_balance)
                .set((
                    balance::person_id.eq(&existing_balance.person_id),
                    balance::value.eq(total),
                ))
                .get_result::<Balance>(connection)?;
            Ok(update_balance)
        }
        Err(_) => {
            let new_balance = diesel::insert_into(balance_table)
                .values(
                    &(NewBalance {
                        person_id: person.id,
                        value: value,
                    }),
                )
                .get_result::<Balance>(connection)?;
            Ok(new_balance)
        }
    }
}

pub fn list_movements(
    connection: &mut SqliteConnection,
    person: &Person,
) -> Result<Vec<Movement>, ControllerError> {
    Ok(Movement::belonging_to(&person)
        .select(Movement::as_select())
        .load(connection)?)
}

pub fn get_currencies(connection: &mut SqliteConnection) -> Result<Vec<String>, ControllerError> {
    let currencies = person::person
        .select(person::currency)
        .distinct()
        .load::<String>(connection)?;
    Ok(currencies)
}
