use bevy::prelude::*;

pub mod bundle_effect;

pub fn plugin(app: &mut App) {
    app.add_plugins(bundle_effect::plugin);
}
