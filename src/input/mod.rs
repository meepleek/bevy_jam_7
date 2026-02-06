use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub mod menu;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((EnhancedInputPlugin, player::plugin));
}
