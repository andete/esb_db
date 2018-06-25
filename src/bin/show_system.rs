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
        println!("system:      {}", s.name);
        println!("security:    {}", a.name);
        println!("permit:      {}", s_o(s.needs_permit));
        println!("reserve:     {}", r.name);

        let sp = {
            use esb_db::schema::system_power::dsl::*;
            system_power
                .filter(system_id.eq(s.id))
                .order(stamp.desc())
                .inner_join(esb_db::schema::allegiance::table)
                .inner_join(esb_db::schema::power_state::table)
                .first::<(SystemPower,Allegiance,PowerState)>(&connection)
                .optional()
                .expect("Error loading system power")
        };
        if let Some((_sp,al,po)) = sp {
            println!("allegiance:  {}", al.name);
            println!("power_state: {}", po.name);
        }
        println!("");
        let presence = {
            use esb_db::schema::presence::dsl::*;
            let p = presence
                .filter(system_id.eq(s.id))
                .order(stamp.desc())
                .first::<Presence>(&connection)
                .optional()
                .expect("Error loading first presence");
            if let Some(p) = p {
                presence
                    .filter(system_id.eq(s.id))
                    .filter(stamp.eq(p.stamp))
                    .order(influence.desc())
                    .inner_join(esb_db::schema::state::table)
                    .inner_join(esb_db::schema::faction::table)
                    .limit(16)
                    .load::<(Presence,State,Faction)>(&connection)
                    .expect("Error loading presence")
            } else {
                vec![]
            }
        };
        for (p,s,f) in presence {
            println!("{} {}: {}", p.influence.unwrap_or(0.0), f.name, s.name);
        }
    } else {
        println!("Not found.");
    }
}
