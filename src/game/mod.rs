use bevy::{math::I16Vec2, prelude::*};

use crate::{
    game::pause::Gameplay,
    utils::{fade_state::FadeStatePlugin, state::HideOnStatePlugin},
};

mod card;
mod enemy;
mod grid;
mod heat;
mod level;
mod movement;
mod pause;
mod player;
mod shop;
mod turn;
mod ui;

pub type Coords = I16Vec2;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::card::card::*;
    pub use super::card::effect::*;
    pub use super::card::effect::{card_effect::*, temp::*, tile_effect::*};
    pub use super::card::pile::*;
    pub use super::enemy::*;
    pub use super::grid::grid::*;
    pub use super::grid::tile::*;
    pub use super::level::*;
    pub use super::movement::*;
    pub use super::pause::*;
    pub use super::player::*;
    pub use super::{Coords, GameplayPhase};
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        heat::plugin,
        movement::plugin,
        shop::plugin,
        pause::plugin,
        player::plugin,
        enemy::plugin,
        grid::plugin,
        card::plugin,
        turn::plugin,
        ui::plugin,
        HideOnStatePlugin::<GameplayPhase>::default(),
        FadeStatePlugin::<GameplayPhase>::default(),
    ))
    .add_sub_state::<GameplayPhase>();
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(Gameplay = Gameplay)]
pub enum GameplayPhase {
    #[default]
    LevelSpawn,
    Gameplay,
    Shop,
    #[allow(dead_code)]
    Win,
    #[allow(dead_code)]
    GameOver,
}
