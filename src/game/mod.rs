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
    .add_sub_state::<GameplayPhase>();
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(Gameplay = Gameplay)]
pub enum GameplayPhase {
    #[default]
    LevelSpawn,
    Gameplay,
    // Score,
}
