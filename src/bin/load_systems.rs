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
    let json_systems:Vec<json::System> = serde_json::from_reader(f).unwrap();
    info!("{} systems loaded into memory.", json_systems.len());
    
    use esb_db::schema::system::dsl::*;
    use esb_db::schema::system;

    let connection = establish_connection();
    let mut c:i32 = 0;
    for json_system in json_systems {
        
        let s:System = json_system.into();

        // TODO: check if controlling_minor_faction_id exists in faction table, if not, replace by None before attempting to insert
        // to avoid insert or update on table "system" violates foreign key constraint "system_controlling_minor_faction_id_fkey"
        
        let results = system.filter(id.eq(s.id))
            .limit(1)
            .load::<System>(&connection)
            .expect("Error loading system");

        if results.is_empty() {
            let _:System = diesel::insert_into(system::table)
                .values(&s)
                .get_result(&connection)
                .expect("Error saving system");
            c += 1;
        } else {
        }
    }
    info!("{} systems stored into the database.", c);
}
