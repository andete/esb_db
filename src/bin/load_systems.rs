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
    
    use esb_db::schema::system;
    
    let connection = establish_connection();
    let mut c_stored:i32 = 0;
    let mut c_updated:i32 = 0;
    for json_system in json_systems {
        
        let controlling_minor_faction_id = json_system.controlling_minor_faction_id;
        let controlling_minor_faction_name = json_system.controlling_minor_faction.clone();
        let power_state_id = json_system.power_state_id;
        let power_state_name = json_system.power_state.clone();
        let allegiance_id = json_system.allegiance_id;
        let power_name = json_system.allegiance.clone();
        let mut s:System = json_system.into();

        let results = {
            use esb_db::schema::system::dsl::*;
            system.filter(id.eq(s.id))
                .limit(1)
                .load::<System>(&connection)
                .expect("Error loading system")
        };

        let system_exists = !results.is_empty();

        if !system_exists {
            //info!("Inserting: {:?}", s);
            diesel::insert_into(system::table)
                .values(&s)
                .execute(&connection)
                .expect("Error saving system");
            c_stored += 1;
        } else {
            let existing_system = &results[0];
            if existing_system.updated_at < s.updated_at || force {
                use esb_db::schema::system::dsl::*;
                let _:System = diesel::update(system.filter(id.eq(s.id)))
                    .set(&s)
                    .get_result(&connection)
                    .expect("Error updating system");
                c_updated += 1;
            }
        }

        // if we know about the controlling faction
        // check what was the last known controlling faction
        // and add new entry if it changed
        if Faction::exists(&connection, controlling_minor_faction_id).expect("Error finding faction") {
            use esb_db::schema::controlling::dsl::*;
            let results = controlling.filter(system_id.eq(s.id))
                .order(stamp.desc())
                .limit(1)
                .load::<Controlling>(&connection)
                .expect("Error loading controlling info");
            let (insert,first) = if results.is_empty() {
                (true,true)
            } else {
                let res = results.iter().next().unwrap();
                // check stamp as well to see if it is newer?
                (res.faction_id != controlling_minor_faction_id, false)
            };
            if insert {
                let c = ControllingInsert {
                    stamp:s.updated_at.unwrap(),
                    system_id:s.id,
                    faction_id:controlling_minor_faction_id,
                };
                diesel::insert_into(esb_db::schema::controlling::table)
                    .values(&c)
                    .execute(&connection)
                    .expect("Error saving controlling");
                if !first {
                    let name = controlling_minor_faction_name.unwrap_or("None".into());
                    info!("System {} new controlling minor faction {}.", s.name, name);
                }
            }
        }

        // check if the Power state has been changed and store it if needed
        {
            use esb_db::schema::system_power::dsl;
            let results = dsl::system_power.filter(dsl::system_id.eq(s.id))
                .order(dsl::stamp.desc())
                .limit(1)
                .load::<SystemPower>(&connection)
                .expect("Error loading system power info");
            let (insert,first) = if results.is_empty() {
                (true,true)
            } else {
                let res:&SystemPower = results.iter().next().unwrap();
                // check stamp as well to see if it is newer?
                let changed = res.allegiance_id != allegiance_id
                    || res.power_state_id != power_state_id;
                (changed, false)
            };
            if insert {
                let c = SystemPowerInsert {
                    stamp:s.updated_at.unwrap(),
                    system_id:s.id,
                    power_state_id: power_state_id,
                    allegiance_id: allegiance_id,
                };
                diesel::insert_into(esb_db::schema::system_power::table)
                    .values(&c)
                    .execute(&connection)
                    .expect("Error saving system_power");
                if !first {
                    let name = power_name.unwrap_or("None".into());
                    let state = power_state_name.unwrap_or("None".into());
                    info!("System {} new power state {}:{}", s.name, name, state);
                }
            }
        }
    }
    info!("{} systems stored.", c_stored);
    info!("{} systems updated.", c_updated);
}
