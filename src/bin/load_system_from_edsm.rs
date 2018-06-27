// (c) 2018 Joost Yervante Damad <joost@damad.be>

extern crate esb_db;
extern crate diesel;
extern crate badlog;
extern crate clap;
extern crate reqwest;
extern crate serde_json;

use esb_db::*;
use esb_db::models::*;
use diesel::prelude::*;

fn main() {

    badlog::minimal(Some("INFO"));
    let a = clap::App::new("load_system_from_edsm")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .arg(clap::Arg::with_name("NAME")
             .required(true)
             .index(1)
             .help("system name"))
        ;
    
    let m = a.get_matches();
    let n = m.value_of("NAME").unwrap();


    let connection = establish_connection();
    
    let system_result = {
        use esb_db::schema::system::dsl::*;
        system
            .filter(name.eq(n))
            .first::<System>(&connection)
            .optional()
            .expect("Error loading system")
    };

    let edsm_id = system_result
        .expect("system not found")
        .edsm_id.expect("edsm id missing");

    let url = format!("https://www.edsm.net/api-system-v1/factions/?systemId={}&showHistory=1", edsm_id);
    let client = reqwest::Client::new();
    let res = client.get(&url).send()
        .expect("IO error getting data from edsm")
        .text().expect("expecting body text in response");

    /* let res = include_str!("../../edsm_azrael_hist_f.json"); */

    let system:esb_db::edsm::System = serde_json::from_str(&res).unwrap();
    println!("system: {:?}", system);
}
