use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use diesel::result::ConnectionResult;

pub fn establish_connection() -> ConnectionResult<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
}
