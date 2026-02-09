use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;
