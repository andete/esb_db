extern crate esb_db;
extern crate diesel;
extern crate badlog;
extern crate clap;

use esb_db::*;
use esb_db::models::*;
use diesel::prelude::*;

fn main() {


    badlog::minimal(Some("DEBUG"));
    let a = clap::App::new("show_system")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joost Yervante Damad <joost@damad.be>")
        .arg(clap::Arg::with_name("NAME")
             .required(true)
             .index(1)
             .help("system name"))
        ;
    
    let m = a.get_matches();
    let n = m.value_of("NAME").unwrap();
    
    use esb_db::schema::system::dsl::*;
    use esb_db::schema::allegiance;

    let connection = establish_connection();
    let results = system
        .inner_join(allegiance::table)
        .filter(name.eq(n))
        .limit(1)
        .load::<(System,Allegiance)>(&connection)
        .expect("Error loading systems");

    if results.len() > 0 {
        for (s,a) in results {
            println!("{:?} {:?}", s, a);
        }
    } else {
        println!("Not found.");
    }
}
