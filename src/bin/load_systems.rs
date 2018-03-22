extern crate badlog;
#[macro_use]
extern crate log;
extern crate clap;
extern crate diesel;
extern crate esb_db;
extern crate serde_json;

use self::esb_db::*;
use self::models::*;
use self::diesel::prelude::*;

use std::fs::File;

fn main() {
    use esb_db::schema::state::dsl::*;
    badlog::minimal(Some("INFO"));
    let a = clap::App::new("load_systems")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .arg(clap::Arg::with_name("FILENAME")
             .required(true)
             .index(1)
             .help("JSON file name"));

    let m = a.get_matches();
    let n = m.value_of("FILENAME").unwrap();
    
    let f = File::open(&n).unwrap();
    let c:Vec<json::System> = serde_json::from_reader(f).unwrap();
    info!("systems: {}", c.len());
    /*
    let connection = establish_connection();
    let results = state
        .limit(50)
        .load::<State>(&connection)
        .expect("Error loading states");

    println!("Displaying {} states", results.len());
    for x in results {
        println!("{:?}", x);
    }
     */
}
