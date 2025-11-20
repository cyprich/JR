use std::env;

use diesel::{Connection, RunQueryDsl, SqliteConnection};
use dotenvy::dotenv;

use crate::{models::TaskDb, task::Task};
use diesel::associations::HasTable;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL has to be set");

    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Error conecting to {}", url))
}
