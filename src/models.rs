use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::pg::PgConnection;

use super::schema::controlling;
use super::schema::faction;
use super::schema::faction_state;
use super::schema::system;
use super::schema::system_power;

#[derive(Debug, Queryable)]
pub struct Allegiance {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Economy {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Government {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Power {
    pub id: i32,
    pub name: String,
    pub allegiance_id: i32,
}

#[derive(Debug, Queryable)]
pub struct PowerState {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct ReserveType {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct Security {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable)]
pub struct State {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
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

#[derive(Debug, Queryable, Insertable, AsChangeset)]
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

#[derive(Debug,Queryable,Associations)]
#[belongs_to(System)]
#[table_name="controlling"]
pub struct Controlling {
    pub id:i32,
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

#[derive(Debug,Queryable,Associations)]
#[belongs_to(System)]
#[table_name="system_power"]
pub struct SystemPower {
    pub id:i32,
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub allegiance_id: Option<i32>,
    pub power_state_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name="system_power"]
pub struct SystemPowerInsert {
    pub stamp: DateTime<Utc>,
    pub system_id: i32,
    pub allegiance_id: Option<i32>,
    pub power_state_id: Option<i32>,
}

#[derive(Debug,Queryable,Associations)]
#[belongs_to(Faction)]
#[table_name="faction_state"]
pub struct FactionState {
    pub id:i32,
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


impl Faction {
    pub fn exists<T>(connection:&PgConnection, faction_id:T) -> QueryResult<Option<Faction>> where T:Into<Option<i32>> {
        if let Some(faction_id) = faction_id.into() {
            use schema::faction::dsl::{faction,id};
            let result = faction.filter(id.eq(faction_id))
                .limit(1)
                .first::<Faction>(connection).optional()?;
            Ok(result)
        } else {
            Ok(None)
        }
    }
}
