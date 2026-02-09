use bevy::{math::I16Vec2, prelude::*};

use crate::game::pause::Gameplay;

pub mod action;
pub mod card;
pub mod card_effect;
pub mod die;
pub mod grid;
pub mod level;
pub mod pause;
pub mod pile;
pub mod player;
pub mod tile;

pub type Coords = I16Vec2;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        pause::plugin,
        player::plugin,
        tile::plugin,
        grid::plugin,
        die::plugin,
        pile::plugin,
        card::plugin,
        card_effect::plugin,
        action::plugin,
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
