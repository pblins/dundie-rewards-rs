use diesel::result::{ConnectionError, Error};
use passwords::hasher;

use crate::database;
use crate::database::models::{Person, User};
use crate::security::{generate_salt, verify_password};

#[derive(Debug)]
pub enum AuthenticationError {
    DatabaseConnection(ConnectionError),
    Database(Error),
    UserNotAuthenticated,
    AccessDenied,
    Controller(database::controller::ControllerError),
}

impl From<ConnectionError> for AuthenticationError {
    fn from(value: ConnectionError) -> Self {
        Self::DatabaseConnection(value)
    }
}

impl From<database::controller::ControllerError> for AuthenticationError {
    fn from(value: database::controller::ControllerError) -> Self {
        Self::Controller(value)
    }
}

pub fn get_password_hash(password: &String) -> String {
    let salt = generate_salt();
    let hashed = hasher::bcrypt(10, &salt, password).unwrap();

    hashed.map(|x| x.to_string()).join("")
}

pub fn authenticate_user(
    username: &String,
    password: &String,
    requires_superuser: bool,
) -> Result<(Person, User), AuthenticationError> {
    let mut connection = database::connection::establish_connection()?;

    match database::controller::user_exists(&mut connection, username) {
        Some(user) => {
            let verified_user = verify_password(password, &user.password);

            if requires_superuser {
                if user.superuser {
                    if verified_user {
                        Ok((
                            database::controller::query_person_by_id(
                                &mut connection,
                                user.person_id,
                            )?,
                            user,
                        ))
                    } else {
                        Err(AuthenticationError::UserNotAuthenticated)
                    }
                } else {
                    Err(AuthenticationError::AccessDenied)
                }
            } else {
                if verified_user {
                    Ok((
                        database::controller::query_person_by_id(&mut connection, user.person_id)?,
                        user,
                    ))
                } else {
                    Err(AuthenticationError::UserNotAuthenticated)
                }
            }
        }
        None => Err(AuthenticationError::Database(Error::NotFound)),
    }
}
