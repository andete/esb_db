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
    

    let connection = establish_connection();
    
    let system_result = {
        use esb_db::schema::system::dsl::*;
        system
            .inner_join(esb_db::schema::security::table)
            .inner_join(esb_db::schema::reserve_type::table)
            .filter(name.eq(n))
            .first::<(System,Security,ReserveType)>(&connection)
            .optional()
            .expect("Error loading systems")
    };

    if let Some((s,a,r)) = system_result {
        println!("system:   {}", s.name);
        println!("security: {}", a.name);
        println!("permit:   {}", s_o(s.needs_permit));
        println!("reserve:  {}", r.name);
    } else {
        println!("Not found.");
    }
}
