use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Deserializer};

use models;

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

impl Into<models::System> for System {
    fn into(self) -> models::System {
        models::System {
            id:self.id,
            name:self.name,
            security_id:self.security_id,
            needs_permit:self.needs_permit,
            x:self.x,
            y:self.y,
            z:self.z,
            simbad_ref:self.simbad_ref,
            reserve_type_id:self.reserve_type_id,
            is_populated:self.is_populated,
            edsm_id:self.edsm_id,
            updated_at:Some(self.updated_at),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinorFactionPresence {
    pub state_id:Option<i32>,
    pub influence:Option<f64>,
    pub minor_faction_id:i32,
    pub state:Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Faction {
    pub id:i32,
    pub name:String,
    pub allegiance_id: Option<i32>,
    pub allegiance: Option<String>,
    pub state_id: Option<i32>,
    pub state: Option<String>,
    pub government_id: Option<i32>,
    pub government: Option<String>,
    pub home_system_id: Option<i32>,
    pub is_player_faction: bool,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

impl Into<models::Faction> for Faction {
    fn into(self) -> models::Faction {
        models::Faction {
            id:self.id,
            name:self.name,
            allegiance_id:self.allegiance_id,
            government_id:self.government_id,
            home_system_id:self.home_system_id,
            is_player_faction:self.is_player_faction,
            updated_at:self.updated_at,
        }
    }
}

fn deserialize_datetime<'de, D>(deserializer:D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let i = i64::deserialize(deserializer)?;
    Ok(Utc.timestamp(i, 0))
}
