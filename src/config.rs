use crate::components::Components;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Data {
    provider: bool,
    player_id: bool,
    player_state: bool,
    map: bool,
    map_round_wins: bool,
    player_match_stats: bool,
    player_weapons: bool,
    round: bool,
    allgrenades: bool,
    allplayers_id: bool,
    allplayers_match_state: bool,
    allplayers_position: bool,
    allplayers_states: bool,
    allplayers_weapons: bool,
    bomb: bool,
    phase_countdowns: bool,
    player_position: bool,
}

impl From<&[Components]> for Data {
    fn from(components: &[Components]) -> Self {
        Self {
            provider: components.contains(&Components::Provider),
            player_id: components.contains(&Components::PlayerId),
            player_state: components.contains(&Components::PlayerState),
            player_match_stats: components.contains(&Components::PlayerMatchStats),
            player_weapons: components.contains(&Components::PlayerWeapons),
            player_position: components.contains(&Components::PlayerPosition),
            map: components.contains(&Components::Map),
            map_round_wins: components.contains(&Components::MapRoundWins),
            round: components.contains(&Components::Round),
            bomb: components.contains(&Components::Bomb),
            phase_countdowns: components.contains(&Components::PhaseCountdowns),
            allgrenades: components.contains(&Components::AllGrenages),
            allplayers_id: components.contains(&Components::AllPlayersId),
            allplayers_states: components.contains(&Components::AllPlayersStates),
            allplayers_match_state: components.contains(&Components::AllPlayersMatchStats),
            allplayers_weapons: components.contains(&Components::AllPlayersWeapons),
            allplayers_position: components.contains(&Components::AllPlayersPosition),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename = "Created with csgo-gsi-builder")]
pub struct Config {
    /// Service name
    ///
    /// Note: it shouldn't contain any spaces or special chars except `_`
    #[serde(skip_serializing)]
    pub name: String,
    pub uri: String,
    pub timeout: f64,
    pub buffer: f64,
    pub throttle: f64,
    pub heartbeat: f64,
    pub data: Data,
}

impl Default for Config {
    fn default() -> Self {
        let components: &[Components] = &[];
        Self {
            name: String::from("unnamed"),
            uri: String::from("127.0.0.1:3000"),
            timeout: 1.1,
            buffer: 0.1,
            throttle: 1.0,
            heartbeat: 30.0,
            data: Data::from(components),
        }
    }
}
