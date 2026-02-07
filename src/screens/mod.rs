//! The game's main screen states and transitions between them.

mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((loading::plugin, splash::plugin, title::plugin));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay {
        paused: bool,
    },
}
impl Screen {
    pub fn playing() -> Self {
        Self::Gameplay { paused: false }
    }

    pub fn paused() -> Self {
        Self::Gameplay { paused: true }
    }
}
