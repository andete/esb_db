use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub id: i32,
    pub name: String,
    pub population:Option<i64>,
    pub allegiance_id: Option<i32>,
    pub allegiance:Option<String>,
    pub state_id: Option<i32>,
    pub state:Option<String>,
    pub government_id: Option<i32>,
    pub government:Option<String>,
    pub security_id: Option<i32>,
    pub security:Option<String>,
    pub primary_economy_id: Option<i32>,
    pub primary_economy:Option<String>,
    pub needs_permit: Option<bool>,
    pub power_state_id: Option<i32>,
    pub power_state:Option<String>,
    pub power:Option<String>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub simbad_ref: Option<String>,
    pub controlling_minor_faction_id: Option<i32>,
    pub controlling_minor_faction: Option<String>,
    pub reserve_type_id: Option<i32>,
    pub reserve_type:Option<String>,
    pub is_populated: Option<bool>,
    pub edsm_id: Option<i32>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
    pub minor_faction_presences:Vec<MinorFactionPresence>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinorFactionPresence {
    pub state_id:Option<i32>,
    pub influence:Option<f64>,
    pub minor_faction_id:i32,
    pub state:Option<String>,
}

fn deserialize_datetime<'de, D>(deserializer:D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let i = i64::deserialize(deserializer)?;
    Ok(Utc.timestamp(i, 0))
}
