use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub mod focus;
pub mod menu;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        EnhancedInputPlugin,
        player::plugin,
        menu::plugin,
        focus::plugin,
    ));
}
