// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate esb_db;
extern crate diesel;

use self::esb_db::*;
use self::model::*;
use self::diesel::prelude::*;

fn main() {
    use esb_db::schema::state::dsl::*;

    let connection = establish_connection();
    let results = state
        .limit(50)
        .load::<DbState>(&connection)
        .expect("Error loading states");

    println!("Displaying {} states", results.len());
    for x in results {
        println!("{:?}", x);
    }
}
