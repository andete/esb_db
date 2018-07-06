// (c) 2018 Joost Yervante Damad <joost@damad.be>

use chrono::{TimeZone, Utc};
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use edsm::System as EdsmSystem;
use model::*;

pub fn process_edsm_system(connection:&PgConnection, system:&EdsmSystem) -> QueryResult<()> {
    // 1. Update controlling
    //
    // EDSM system info has latest controlling faction info
    // however there is _no_ history and no timestamp info
    // so we get the last update timestamp from the factions list
    let last_update_opt = system.last_update();
    if last_update_opt.is_none() {
        info!("No faction info exists for {}", system.name);
        return Ok(())
    }
    let edsm_update = last_update_opt.unwrap();
    let edsm_stamp = Utc.timestamp(edsm_update, 0);

    // get db controlling faction by id
    let faction_name = &system.controlling_faction.name;
    let faction_id = system.controlling_faction.id;
    let db_faction_opt = Faction::exists(connection, faction_id)?;
    if db_faction_opt.is_none() {
        info!("Faction not known in db: {}", faction_name);
        return Ok(())
    }

    // get db system by id
    let db_system_opt = System::by_edsm_id(connection, system.id)?;
    if db_system_opt.is_none() {
        info!("System not known in db: {}", system.name);
        return Ok(())
    }
    let db_system = db_system_opt.unwrap();

    let mut insert_controlling = false;
    if let Some(controlling) = db_system.last_controlling(connection)? {
        // TODO: deal with noise at the border of the tick
        if controlling.stamp < edsm_stamp && controlling.faction_id != Some(faction_id) {
            insert_controlling = true;
        }
    } else {
        insert_controlling = true;
    }
    if insert_controlling {
        let ci = ControllingInsert {
            stamp:edsm_stamp,
            system_id:db_system.id,
            faction_id:Some(faction_id),
        };
        diesel::insert_into(::schema::controlling::table)
            .values(&ci)
            .execute(connection)?;
        info!("Controlling faction updated in {}: {}", system.name, faction_name);
    }
    
    // 2. Update faction presence (state, pending, recovery)
    // for now only insert latest
    // at some point we may want to inspect the whole history as well
    for faction in &system.factions {
        let state = State::by_name(connection, &faction.state)?.unwrap();
        let db_faction_opt = Faction::exists(connection, faction.id)?;
        if db_faction_opt.is_none() {
            info!("Faction not known in db: {}", faction_name);
            continue;
        }
        // let db_faction = db_faction_opt.unwrap();
        let mut insert_presence = true;
        if let Some(presence) = db_system.last_presence(connection, faction_id)? {
            insert_presence =
                presence.stamp < edsm_stamp
                && (presence.state_id != Some(state.id) ||
                    presence.influence != Some(faction.influence))
        }
        if insert_presence {
        let ci = PresenceInsert {
            stamp:edsm_stamp,
            system_id:system.id,
            faction_id:faction.id,
            state_id:Some(state.id),
            influence:Some(faction.influence),
        };
        diesel::insert_into(::schema::presence::table)
            .values(&ci)
            .execute(connection)?;
        info!("Faction presence updated: {}: {} {} {}", system.name, faction_name, faction.state, faction.influence);
        }
    }
    Ok(())
}
