use std::env;

use diesel::{Connection, SqliteConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL has to be set");
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error connecting to {}", url))
}
