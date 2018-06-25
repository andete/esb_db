extern crate esb_db;
extern crate diesel;
extern crate badlog;
extern crate clap;

use esb_db::*;
use esb_db::models::*;
use diesel::prelude::*;

use std::fmt::Display;

fn s_o<'a, T:Display>(x:Option<T>) -> String {
    if let Some(y) = x {
        format!("{}", y)
    } else {
        "None".into()
    }
}

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

    let connection = establish_connection();
    let results = system
        .inner_join(esb_db::schema::security::table)
        .inner_join(esb_db::schema::reserve_type::table)
        .filter(name.eq(n))
        .limit(1)
        .load::<(System,Security,ReserveType)>(&connection)
        .expect("Error loading systems");

    if results.len() > 0 {
        let (s,a,r) = &results[0];
        println!("System:   {}", s.name);
        println!("Security: {}", a.name);
        println!("permit:   {}", s_o(s.needs_permit));
        println!("reserve:  {}", r.name);
    } else {
        println!("Not found.");
    }
}
