// (c) 2018 Joost Yervante Damad <joost@damad.be>

use std::collections::HashMap;

use model::State;

// public static String factionUrl = 'https://www.edsm.net/api-system-v1/    factions/?systemId=${edsmSystemId}&showHistory=1"

// Azrael:
// https://www.edsm.net/api-system-v1/factions/?systemId=11296&showHistory=1

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct System {
    pub id:i32,
    pub id64:i64,
    pub name:String,
    #[serde(rename="controllingFaction")]
    pub controlling_faction:ControllingFactionInfo,
    pub url:String,
    pub factions:Vec<FactionInfo>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FactionInfo {
    pub id:i32,
    pub name:String,
    pub government:String,
    pub allegiance: String,
    #[serde(rename="isPlayer")]
    pub is_player: bool,
    pub influence:f32,
    pub state: State,
    #[serde(rename="stateHistory")]
    pub state_history:HashMap<i64,String>,
    #[serde(rename="pendingStates")]
    pub pending_states: Vec<StateTrend>,
    #[serde(rename="pendingStatesHistory")]
    pub pending_states_history:HashMap<i64,Vec<StateTrend>>,
    #[serde(rename="recoveringStates")]
    pub recovering_states: Vec<StateTrend>,
    #[serde(rename="recoveringStatesHistory")]
    pub recovering_states_history:HashMap<i64,Vec<StateTrend>>,
    #[serde(rename="influenceHistory")]
    pub influence_history:HashMap<i64,f32>,
    #[serde(rename="lastUpdate")]
    pub last_update: i64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StateTrend {
    pub state:State,
    pub trend:i8,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControllingFactionInfo {
    pub allegiance:String,
    pub government:String,
    pub id:i32,
    pub name:String,
}

impl System {
    pub fn last_update(&self) -> Option<i64> {
        self.factions.iter().map(|f| f.last_update).max()
    }
}
