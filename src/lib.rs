// (c) 2018 Joost Yervante Damad <joost@damad.be>

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// println!("{}", diesel::query_builder::debug_query::<diesel::pg::Pg, _>(&query));

pub mod calculate;
pub mod edsm;
pub mod eddb;
pub mod model;
pub mod schema;
