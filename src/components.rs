/// Components available.
///
/// You can find exemples of what each component does [here] _([archive])_.
///
/// [here]: https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/
/// [archive]: https://web.archive.org/web/20220906050651/https://www.reddit.com/r/GlobalOffensive/comments/cjhcpy/game_state_integration_a_very_large_and_indepth/
#[derive(Eq, PartialEq)]
pub enum Components {
    /// General info about the client being listened to, the most import one
    /// being it's SteamID64.
    Provider,
    /// General info about the player being spectated (yourself if you are
    /// playing and alive).
    PlayerId,
    /// Current state about the player being spectated (i.e. health, armor,
    /// money, ...).
    PlayerState,
    /// Match statistics about the player being spectated (i.e. kills, assists, ...).
    PlayerMatchStats,
    /// Weapons held by the player being spectated.
    PlayerWeapons,
    /// Map and game information (i.e. map name, round number, teams score, ...).
    Map,
    /// Winning condition of every round played.
    MapRoundWins,
    /// Current round information, similar to [`Components::PhaseCountdowns`].
    Round,
    /// State of every grenades on the map (i.e. coordinates, lifetime, velocity,
    /// ...).
    AllGrenages,
    /// See [`Components::PlayerId`].
    AllPlayersId,
    /// See [`Components::PlayerMatchStats`].
    AllPlayersMatchStats,
    /// See [`Components::PlayerPosition`].
    AllPlayersPosition,
    /// See [`Components::PlayerState`].
    AllPlayersStates,
    /// See [`Components::PlayerWeapons`].
    AllPlayersWeapons,
    /// State, position, and other information about the bomb.
    Bomb,
    /// Current phase of the round information, similar to [`Components::Round`].
    PhaseCountdowns,
    /// Coordinates of the player being spectated.
    PlayerPosition,
}

impl Components {
    /// Components containing information about the currently spectated player
    /// (or the one playing if he's alive).
    pub const PLAYER_POV: &[Components] = &[
        Components::Provider,
        Components::PlayerId,
        Components::PlayerState,
        Components::Map,
        Components::MapRoundWins,
        Components::PlayerMatchStats,
        Components::PlayerWeapons,
        Components::Round,
    ];

    /// Components only available for spectators.
    pub const SPECTATOR_POV: &[Components] = &[
        Components::AllGrenages,
        Components::AllPlayersId,
        Components::AllPlayersMatchStats,
        Components::AllPlayersPosition,
        Components::AllPlayersStates,
        Components::AllPlayersWeapons,
        Components::Bomb,
        Components::PhaseCountdowns,
        Components::PlayerPosition,
    ];

    /// All the components.
    pub const ALL: &[Components] = &[
        Components::Provider,
        Components::PlayerId,
        Components::PlayerState,
        Components::Map,
        Components::MapRoundWins,
        Components::PlayerMatchStats,
        Components::PlayerWeapons,
        Components::Round,
        Components::AllGrenages,
        Components::AllPlayersId,
        Components::AllPlayersMatchStats,
        Components::AllPlayersPosition,
        Components::AllPlayersStates,
        Components::AllPlayersWeapons,
        Components::Bomb,
        Components::PhaseCountdowns,
        Components::PlayerPosition,
    ];
}
