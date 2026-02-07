use bevy::{
    input_focus::{
        AutoFocus, InputFocus,
        directional_navigation::{DirectionalNavigationError, DirectionalNavigationPlugin},
    },
    math::CompassOctant,
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigator,
};
use bevy_enhanced_input::prelude::*;
use tiny_bail::{or_continue, or_return};

use crate::{input::focus::UiFocus, theme::prelude::InteractionPalette};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DirectionalNavigationPlugin)
        .init_resource::<InputFocus>()
        .add_input_context::<MenuInputCtx>()
        .add_observer(apply_menu_movement)
        .add_observer(apply_confirm)
        .add_observer(handle_interaction_btn_click);
}

#[derive(Component)]
pub struct MenuInputCtx;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MenuMovement;

#[derive(InputAction)]
#[action_output(bool)]
pub struct MenuConfirm;

#[derive(EntityEvent)]
pub struct ButtonClick(Entity);

pub fn menu_input() -> impl Bundle {
    (
        MenuInputCtx,
        actions!(MenuInputCtx[
            (
                Action::<MenuMovement>::new(),
                Pulse::new(0.25),
                DeadZone::default(),
                Bindings::spawn((
                    // Bindings like WASD or sticks are very common,
                    // so we provide built-in `SpawnableList`s to assign all keys/axes at once.
                    Cardinal::wasd_keys(),
                    Cardinal::arrows(),
                    Cardinal::dpad(),
                    Axial::left_stick(),
                )),
            ),
            (
                Action::<MenuConfirm>::new(),
                bindings![KeyCode::Space, KeyCode::Enter, GamepadButton::South]
            )
        ]),
    )
}

fn apply_menu_movement(
    movement: On<Fire<MenuMovement>>,
    mut navigator: AutoDirectionalNavigator,
    autofocus_e: Single<Entity, With<AutoFocus>>,
) {
    let normalized = movement.value.signum();
    let nav_result = match (normalized.x, normalized.y) {
        (_, 1.) => navigator.navigate(CompassOctant::North),
        (_, -1.) => navigator.navigate(CompassOctant::South),
        (1., _) => navigator.navigate(CompassOctant::East),
        (-1., _) => navigator.navigate(CompassOctant::West),
        _ => {
            return;
        }
    };

    if let Err(DirectionalNavigationError::NoNeighborInDirection { .. }) = nav_result {
        warn!("wrapping back to autofocus entity");
        navigator
            .manual_directional_navigation
            .focus
            .set(autofocus_e.entity());
    }
}

fn apply_confirm(_confirm: On<Complete<MenuConfirm>>, mut cmd: Commands, focus: Res<UiFocus>) {
    let e = or_return!(focus.focus()).entity;
    warn!(?e, "confirming!");
    cmd.trigger(ButtonClick(e));
}

fn handle_interaction_btn_click(
    click: On<Pointer<Click>>,
    mut cmd: Commands,
    palette_q: Query<(), With<InteractionPalette>>,
) {
    let e = click.event_target();
    if palette_q.contains(e) {
        cmd.trigger(ButtonClick(e));
    }
}
