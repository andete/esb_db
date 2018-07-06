use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::*;

#[derive(Debug, Identifiable, Queryable)]
#[table_name="allegiance"]
pub struct Allegiance {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="economy"]
pub struct Economy {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="government"]
pub struct Government {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="power"]
pub struct Power {
    pub id: i32,
    pub name: String,
    pub allegiance_id: i32,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="power_state"]
pub struct PowerState {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="reserve_type"]
pub struct ReserveType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="security"]
pub struct Security {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="state"]
pub struct State {
    pub id: i32,
    pub name: String,
}

impl State {
    pub fn by_name(connection:&PgConnection, n:&str) -> QueryResult<Option<State>> {
        use schema::state::dsl::{state,name};
        state.filter(name.eq(n))
            .first::<State>(connection)
            .optional()
    }
}

#[derive(Debug, Identifiable, Queryable, Insertable, AsChangeset)]
#[table_name="system"]
pub struct System {
    pub id: i32,
    pub name: String,
    pub security_id: Option<i32>,
    pub needs_permit: Option<bool>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub reserve_type_id: Option<i32>,
    pub is_populated: Option<bool>,
    pub edsm_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Identifiable, Queryable, Insertable, AsChangeset)]
#[table_name="faction"]
pub struct Faction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: Option<i32>,
    pub government_id: Option<i32>,
    pub home_system_id: Option<i32>,
    pub is_player_faction: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Identifiable, Queryable)]
#[table_name="rich_faction"]
pub struct RichFaction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: Option<i32>,
    pub allegiance: Option<String>,
    pub government_id: Option<i32>,
    pub government: Option<String>,
    pub home_system_id: Option<i32>,
    pub home_system: Option<String>,
    pub is_player_faction: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(System)]
#[table_name="controlling"]
pub struct Controlling {
    pub id:i64,
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub faction_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name="controlling"]
pub struct ControllingInsert {
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub faction_id: Option<i32>,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(System)]
#[table_name="system_power"]
pub struct SystemPower {
    pub id:i64,
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub power_state_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name="system_power"]
pub struct SystemPowerInsert {
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub power_state_id: Option<i32>,
}

// Faction wide state, faction also has a state
// on a system level
#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(Faction)]
#[table_name="faction_state"]
pub struct FactionState {
    pub id:i64,
    pub stamp: DateTime<Utc>,
    pub faction_id: i32,
    pub state_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name="faction_state"]
pub struct FactionStateInsert {
    pub stamp: DateTime<Utc>,
    pub faction_id: i32,
    pub state_id: i32,
}

#[derive(Debug, Identifiable, Queryable, Associations)]
#[belongs_to(System)]
#[table_name="presence"]
pub struct Presence {
    pub id:i64,
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub faction_id: i32,
    pub state_id: Option<i32>,
    pub influence: f32,
}

#[derive(Debug, Insertable)]
#[table_name="presence"]
pub struct PresenceInsert {
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub faction_id: i32,
    pub state_id: Option<i32>,
    pub influence: f32,
}

impl Faction {
    pub fn exists<T>(connection:&PgConnection, faction_id:T) -> QueryResult<Option<Faction>> where T:Into<Option<i32>> {
        if let Some(faction_id) = faction_id.into() {
            use schema::faction::dsl::{faction,id};
            let result = faction.filter(id.eq(faction_id))
                .first::<Faction>(connection)
                .optional()?;
            Ok(result)
        } else {
            Ok(None)
        }
    }

    pub fn last_faction_state(&self, connection:&PgConnection) -> QueryResult<Option<FactionState>> {
        use schema::faction_state::dsl::stamp;
        FactionState::belonging_to(self)
            .order(stamp.desc())
            .first(connection)
            .optional()
    }
    
}

impl System {
    pub fn by_name(connection:&PgConnection, n:&str) -> QueryResult<Option<System>> {
        use schema::system::dsl::{system,name};
        system
            .filter(name.eq(n))
            .first::<System>(connection)
            .optional()
    }

    pub fn by_edsm_id(connection:&PgConnection, id:i32) -> QueryResult<Option<System>> {
        use schema::system::dsl::{system,edsm_id};
        system
            .filter(edsm_id.eq(id))
            .first::<System>(connection)
            .optional()
    }
    
    pub fn last_controlling(&self, connection:&PgConnection) -> QueryResult<Option<Controlling>> {
        use schema::controlling::dsl::stamp;
        Controlling::belonging_to(self)
            .order(stamp.desc())
            .first(connection)
            .optional()
    }
    
    pub fn last_presence(&self, connection:&PgConnection, f_id:i32) -> QueryResult<Option<Presence>> {
        use schema::presence::dsl::{stamp,faction_id};
        Presence::belonging_to(self)
            .filter(faction_id.eq(f_id))
            .order(stamp.desc())
            .first(connection)
            .optional()
    }
}
