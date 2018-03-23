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
    badlog::minimal(Some("DEBUG"));
    let a = clap::App::new("load_systems")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .arg(clap::Arg::with_name("FILENAME")
             .required(true)
             .index(1)
             .help("JSON file name"))
        .arg(clap::Arg::with_name("force")
             .short("f")
             .long("force")
             .help("force overwrite"))
        ;

    let m = a.get_matches();
    let n = m.value_of("FILENAME").unwrap();
    let force = m.is_present("force");
    
    let f = File::open(&n).unwrap();
    let json_systems:Vec<json::System> = serde_json::from_reader(f).unwrap();
    info!("{} systems loaded into memory.", json_systems.len());
    
    use esb_db::schema::system::dsl::*;
    use esb_db::schema::system;
    use esb_db::schema::faction;
    use diesel::dsl::count_star;

    let connection = establish_connection();
    let mut c_stored:i32 = 0;
    let mut c_updated:i32 = 0;
    for json_system in json_systems {
        
        let mut s:System = json_system.into();

        let results = system.filter(id.eq(s.id))
            .limit(1)
            .load::<System>(&connection)
            .expect("Error loading system");

        let system_exists = !results.is_empty();
        
        let faction_exists = if let Some(faction_id) = s.controlling_minor_faction_id {
            faction::dsl::faction.filter(faction::dsl::id.eq(faction_id))
            .select(count_star())
            .first::<i64>(&connection)
                .expect("Error finding faction") > 0
        } else {
            false
        };

        if !faction_exists {
            s.controlling_minor_faction_id = None;
        }

        if !system_exists {
            //info!("Inserting: {:?}", s);
            let _:System = diesel::insert_into(system::table)
                .values(&s)
                .get_result(&connection)
                .expect("Error saving system");
            c_stored += 1;
        } else {
            let existing_system = &results[0];
            if existing_system.updated_at < s.updated_at || force {
                let _:System = diesel::update(system.filter(id.eq(s.id)))
                    .set(&s)
                    .get_result(&connection)
                    .expect("Error updating system");
                c_updated += 1;
            }
        }
    }
    info!("{} systems stored.", c_stored);
    info!("{} systems updated.", c_updated);
}
