#[derive(Eq, PartialEq)]
pub enum Components {
    Provider,
    PlayerId,
    PlayerState,
    Map,
    MapRoundWins,
    PlayerMatchStats,
    PlayerWeapons,
    Round,
    AllGrenages,
    AllPlayersId,
    AllPlayersMatchStats,
    AllPlayersPosition,
    AllPlayersStates,
    AllPlayersWeapons,
    Bomb,
    PhaseCountdowns,
    PlayerPosition,
}

impl Components {
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
