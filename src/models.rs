use chrono::{DateTime, Utc};

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

#[derive(Debug, Queryable)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub allegiance_id: i32,
    pub state_id: i32,
    pub government_id: i32,
    pub security_id: i32,
    pub needs_permit: Option<bool>,
    pub power_state_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub controlling_minor_faction_id: i32,
    pub reserve_type_id: i32,
    pub is_populated: Option<bool>,
    pub edsm_id: i32,
    pub updated_at: Option<DateTime<Utc>>,
}
