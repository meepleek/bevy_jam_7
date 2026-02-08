mod loading;
mod splash;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.add_plugins((loading::plugin, splash::plugin));
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum PauseState {
    #[default]
    Pause,
    Settings,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum MainMenuState {
    #[default]
    Title,
    Settings,
    Credits,
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum Screen {
    #[default]
    Splash,
    MainMenu(MainMenuState),
    Loading,
    Gameplay {
        pause: Option<PauseState>,
    },
}
impl Screen {
    pub fn title() -> Self {
        Self::MainMenu(MainMenuState::Title)
    }

    pub fn playing() -> Self {
        Self::Gameplay { pause: None }
    }

    pub fn paused() -> Self {
        Self::Gameplay {
            pause: Some(PauseState::Pause),
        }
    }
}
