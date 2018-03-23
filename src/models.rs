use chrono::{DateTime, Utc};

use super::schema::system;

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
    pub allegiance_id: Option<i32>,
    pub state_id: Option<i32>,
    pub government_id: Option<i32>,
    pub security_id: Option<i32>,
    pub needs_permit: Option<bool>,
    pub power_state_id: Option<i32>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub controlling_minor_faction_id: Option<i32>,
    pub reserve_type_id: Option<i32>,
    pub is_populated: Option<bool>,
    pub edsm_id: Option<i32>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Queryable)]
pub struct Faction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: i32,
    pub state_id: i32,
    pub government_id: i32,
    pub home_system_id: i32,
    pub is_player_faction: bool,
    pub updated_at: DateTime<Utc>,
}
