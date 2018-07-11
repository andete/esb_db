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

    // get db controlling faction by name in order
    // to get the eddb_id
    let faction_name = &system.controlling_faction.name;
    let db_faction_opt = Faction::by_name(connection, faction_name)?;
    if db_faction_opt.is_none() {
        info!("Faction not known in db: {}", faction_name);
        return Ok(())
    }
    let db_faction = db_faction_opt.unwrap();
    let faction_id = db_faction.id;

    // get db system by edsm_id
    let db_system_opt = System::by_edsm_id(connection, system.edsm_id)?;
    if db_system_opt.is_none() {
        info!("System not known in db: {}", system.name);
        return Ok(())
    }
    let db_system = db_system_opt.unwrap();

    let mut insert_controlling = false;
    if let Some(controlling) = db_system.last_controlling(connection)? {
        // TODO: deal with noise at the border of the tick
        // although... as there is no historic data I wonder if edsm
        // already deals with it; guess this will start to show in
        // practice once data starts to poor in
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
        // get faction by name as edsm faction id
        // does not match eddb faction id
        let db_faction_opt = Faction::by_name(connection, &faction.name)?;
        if db_faction_opt.is_none() {
            info!("Faction not known in db: {}", faction.name);
            continue;
        }
        let db_faction = db_faction_opt.unwrap();
        // 2.1 Presence
        let mut insert_presence = true;
        if let Some(presence) = db_system.last_presence(connection, db_faction.id)? {
            insert_presence =
                presence.stamp < edsm_stamp
                && (presence.state_id != faction.state.id() ||
                    presence.influence != faction.influence)
        }
        if insert_presence {
        let ci = PresenceInsert {
            stamp:edsm_stamp,
            system_id:db_system.id,
            faction_id:db_faction.id,
            state_id:faction.state.id(),
            influence:faction.influence,
        };
        diesel::insert_into(::schema::presence::table)
            .values(&ci)
            .execute(connection)?;
        info!("Faction presence updated: {}: {} {:?} {}", system.name, faction.name, faction.state, faction.influence);
        }
        // 2.2 State, Pending, Recovery
    }
    Ok(())
}
