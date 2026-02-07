use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_computed_state::<Gameplay>()
        .add_computed_state::<Paused>()
        .add_computed_state::<Playing>();
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Gameplay;
impl ComputedStates for Gameplay {
    type SourceStates = Screen;

    fn compute(screen: Screen) -> Option<Self> {
        match screen {
            Screen::Gameplay { .. } => Some(Gameplay),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Paused;
impl ComputedStates for Paused {
    type SourceStates = Screen;

    fn compute(screen: Screen) -> Option<Self> {
        match screen {
            Screen::Gameplay { paused: true } => Some(Paused),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Playing;
impl ComputedStates for Playing {
    type SourceStates = Screen;

    fn compute(screen: Screen) -> Option<Self> {
        match screen {
            Screen::Gameplay { paused: false } => Some(Playing),
            _ => None,
        }
    }
}
