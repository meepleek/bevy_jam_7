use bevy::{math::I16Vec2, prelude::*};

use crate::game::pause::Gameplay;

mod card;
mod grid;
mod level;
mod pause;
mod player;

pub type Coords = I16Vec2;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::card::action::*;
    pub use super::card::card::*;
    pub use super::card::effect::*;
    pub use super::card::pile::*;
    pub use super::grid::grid::*;
    pub use super::grid::tile::*;
    pub use super::level::*;
    pub use super::pause::*;
    pub use super::player::*;
    pub use super::{AiRoundPhase, Coords, GameplayPhase, PlayerRoundPhase, TurnOrder};
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        pause::plugin,
        player::plugin,
        grid::plugin,
        card::plugin,
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
