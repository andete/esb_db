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
    let a = clap::App::new("load_factions")
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
    let json_factions:Vec<json::Faction> = serde_json::from_reader(f).unwrap();
    info!("{} factions loaded into memory.", json_factions.len());
    
    use esb_db::schema::faction::dsl::*;
    use esb_db::schema::faction;

    let connection = establish_connection();
    let mut c_stored:i32 = 0;
    let mut c_updated:i32 = 0;
    for json_faction in json_factions {
        
        let mut s:Faction = json_faction.into();

        let results = faction.filter(id.eq(s.id))
            .limit(1)
            .load::<Faction>(&connection)
            .expect("Error loading faction");

        let faction_exists = !results.is_empty();
        
        if !faction_exists {
            //info!("Inserting: {:?}", s);
            let _:Faction = diesel::insert_into(faction::table)
                .values(&s)
                .get_result(&connection)
                .expect("Error saving faction");
            c_stored += 1;
        } else {
            let existing_faction = &results[0];
            if existing_faction.updated_at < s.updated_at || force {
                let _:Faction = diesel::update(faction.filter(id.eq(s.id)))
                    .set(&s)
                    .get_result(&connection)
                    .expect("Error updating faction");
                c_updated += 1;
            }
        }
    }
    info!("{} factions stored.", c_stored);
    info!("{} factions updated.", c_updated);
}
