//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions,
    input::common_conditions::{input_just_pressed, input_toggle_active},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{prelude::TempChangeAction, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.add_systems(
        Update,
        change_temp::<true>.run_if(input_just_pressed(KeyCode::Digit3)),
    )
    .add_systems(
        Update,
        change_temp::<false>.run_if(input_just_pressed(KeyCode::Digit0)),
    );

    // inspector
    app.add_plugins(EguiPlugin::default()).add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Tab;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn change_temp<const ADD: bool>(mut cmd: Commands) {
    cmd.trigger(TempChangeAction {
        change: if ADD { 1 } else { -1 },
    });
}
