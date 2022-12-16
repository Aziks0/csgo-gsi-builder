use crate::components::Components;

use serde::Serialize;

/// Components to subscribe to.
///
/// The structure shouldn't be used directly, you should prefer creating a slice
/// of [`Components`] and cast it using `into()` or `Data::from()`.
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

/// Configuration for Game State Integration.
///
/// You can find more informations on [VALVE's wiki].
///
/// [VALVE's wiki]: https://developer.valvesoftware.com/wiki/Counter-Strike:_Global_Offensive_Game_State_Integration
#[derive(Serialize, Debug)]
#[serde(rename = "Created with csgo-gsi-builder")]
pub struct Config {
    /// Service name.
    ///
    /// It will be used to create the cfg filename, so it shouldn't contain any
    /// spaces or special chars except `_`.
    #[serde(skip_serializing)]
    pub name: String,

    /// The uri that will receive the payload.
    ///
    /// The game will be making POST requests to this uri.
    pub uri: String,

    /// Game expects an HTTP 2XX response code from its HTTP POST request, and
    /// game will not attempt submitting the next HTTP POST request while a
    /// previous request is still in flight. The game will consider the request
    /// as timed out if a response is not received within so many seconds, and
    /// will re-heartbeat next time with full state omitting any
    /// delta-computation.
    pub timeout: f64,

    /// Because multiple game events tend to occur one after another very
    /// quickly, it is recommended to specify a non-zero buffer. When buffering
    /// is enabled, the game will collect events for so many seconds to report a
    /// bigger delta. For localhost service integration this is less of an issue
    /// and can be tuned to match the needs of the service or set to 0.0 to
    /// disable buffering completely.
    pub buffer: f64,

    /// For high-traffic endpoints this setting will make the game client not
    /// send another request for at least this many seconds after receiving
    /// previous HTTP 2XX response to avoid notifying the service when game
    /// state changes too frequently.
    pub throttle: f64,

    /// Even if no game state change occurs, this setting instructs the game to
    /// send a request so many seconds after receiving previous HTTP 2XX
    /// response. The service can be configured to consider game as offline or
    /// disconnected if it didn't get a notification for a significant period of
    /// time exceeding the heartbeat interval.
    pub heartbeat: f64,

    /// Subscribed [`Components`].
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
