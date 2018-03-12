extern crate bgs_db;
extern crate diesel;

use self::bgs_db::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use bgs_db::schema::state::dsl::*;

    let connection = establish_connection();
    let results = state
        .limit(50)
        .load::<State>(&connection)
        .expect("Error loading states");

    println!("Displaying {} states", results.len());
    for x in results {
        println!("{:?}", x);
    }
}
