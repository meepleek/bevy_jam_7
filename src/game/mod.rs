use bevy::prelude::*;

use crate::game::pause::Gameplay;

mod animation;
pub mod level;
mod movement;
pub mod pause;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        pause::plugin,
        player::plugin,
    ))
    .add_sub_state::<GameplayPhase>()
    .add_sub_state::<TurnOrder>()
    .add_sub_state::<PlayerRoundPhase>()
    .add_sub_state::<AiRoundPhase>();
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(Gameplay = Gameplay)]
pub enum GameplayPhase {
    #[default]
    LevelSpawn,
    Gameplay,
    #[allow(dead_code)]
    Win,
    #[allow(dead_code)]
    GameOver,
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameplayPhase = GameplayPhase::Gameplay)]
pub enum TurnOrder {
    #[default]
    Player,
    Ai,
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(TurnOrder = TurnOrder::Player)]
pub enum PlayerRoundPhase {
    #[default]
    CardSelection,
    CardEffect,
    Cleanup,
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(TurnOrder = TurnOrder::Ai)]
pub enum AiRoundPhase {
    #[default]
    Attack,
    Move,
}
