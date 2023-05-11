use std::collections::HashMap;
use std::fs::File;

use diesel::result::ConnectionError;
use diesel::result::Error;

use crate::database;
use crate::database::controller::ControllerError;
use crate::database::models::{Balance, Movement};
use crate::serializers::{PersonIn, PersonOut};
use crate::utils::exchange::{get_rates, ExchangeError};

#[derive(Debug)]
pub enum CoreError {
    DatabaseConnection(ConnectionError),
    Database(Error),
    Exchange(ExchangeError),
    Controller(ControllerError),
}

impl From<ExchangeError> for CoreError {
    fn from(value: ExchangeError) -> Self {
        Self::Exchange(value)
    }
}

impl From<Error> for CoreError {
    fn from(value: Error) -> Self {
        Self::Database(value)
    }
}

impl From<ConnectionError> for CoreError {
    fn from(value: ConnectionError) -> Self {
        Self::DatabaseConnection(value)
    }
}

impl From<ControllerError> for CoreError {
    fn from(value: ControllerError) -> Self {
        Self::Controller(value)
    }
}

pub fn load(filepath: String) -> Result<Vec<PersonOut>, CoreError> {
    let input = File::open(filepath).expect("Error reading the file");
    let mut result: Vec<PersonOut> = Vec::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input);

    let mut connection = database::connection::establish_connection()?;

    for deserialize_result in rdr.deserialize() {
        let record: PersonIn = deserialize_result.expect("Error converting data");
        let (db_person, created) =
            database::controller::add_person(&mut connection, &record.into())?;

        let person_balance =
            database::controller::query_balance_by_person(&mut connection, &db_person)?;

        result.push(PersonOut {
            name: db_person.name,
            dept: db_person.dept,
            role: db_person.role,
            email: db_person.email,
            currency: db_person.currency,
            created: created,
            balance: person_balance.value,
            value: 0_f32,
        });
    }

    Ok(result)
}

pub fn search(query: &HashMap<String, String>) -> Result<Vec<PersonOut>, CoreError> {
    let mut result: Vec<PersonOut> = Vec::new();

    let mut connection = database::connection::establish_connection()?;
    let people = database::controller::query_person(&mut connection, &query)?;
    let rates = get_rates(database::controller::get_currencies(&mut connection)?)?;

    for person in people {
        let person_balance =
            database::controller::query_balance_by_person(&mut connection, &person)?;
        let rate;

        match rates.get(&person.currency) {
            Some(r) => rate = r,
            None => return Err(CoreError::Exchange(ExchangeError::NotFound)),
        }

        result.push(PersonOut {
            name: person.name.clone(),
            dept: person.dept.clone(),
            role: person.role.clone(),
            email: person.email.clone(),
            currency: person.currency.clone(),
            created: false,
            balance: person_balance.value,
            value: rate.value.parse::<f32>().unwrap() * person_balance.value,
        });
    }

    Ok(result)
}

pub fn move_points(
    value: f32,
    actor: &String,
    query: &HashMap<String, String>,
) -> Result<Vec<PersonOut>, CoreError> {
    let mut result: Vec<PersonOut> = Vec::new();
    let mut connection = database::connection::establish_connection()?;

    let people = database::controller::query_person(&mut connection, query)?;

    for person in &people {
        database::controller::add_movement(&mut connection, person, value, Some(actor.clone()))?;
        search(&query)?.iter().for_each(|p| result.push(p.clone()));
    }

    Ok(result)
}

pub fn get_statement(person_id: i32) -> Result<(Balance, Vec<Movement>), CoreError> {
    let mut connection = database::connection::establish_connection()?;
    let person = database::controller::query_person_by_id(&mut connection, person_id)?;
    let movements = database::controller::list_movements(&mut connection, &person)?;
    let balance = database::controller::query_balance_by_person(&mut connection, &person)?;

    Ok((balance, movements))
}
