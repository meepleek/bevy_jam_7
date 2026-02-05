use bevy::prelude::*;
use bevy_trauma_shake::{Shake, TraumaPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TraumaPlugin)
        .add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d, Shake::default()));
}
