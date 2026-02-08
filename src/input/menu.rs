use bevy::{
    input_focus::{
        InputFocus,
        directional_navigation::{DirectionalNavigationMap, DirectionalNavigationPlugin},
    },
    math::CompassOctant,
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigator,
};
use bevy_enhanced_input::prelude::*;
use tiny_bail::or_return;

use crate::{input::focus::UiFocus, theme::prelude::InteractionPalette};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DirectionalNavigationPlugin)
        .init_resource::<InputFocus>()
        .add_input_context::<MenuInputCtx>()
        .add_systems(Update, add_looping_menu_edges)
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

#[derive(Component)]
pub struct LoopingMenu;

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

fn apply_menu_movement(movement: On<Fire<MenuMovement>>, mut navigator: AutoDirectionalNavigator) {
    let normalized = movement.value.signum();
    let _ = match (normalized.x, normalized.y) {
        (_, 1.) => navigator.navigate(CompassOctant::North),
        (_, -1.) => navigator.navigate(CompassOctant::South),
        (1., _) => navigator.navigate(CompassOctant::East),
        (-1., _) => navigator.navigate(CompassOctant::West),
        _ => {
            return;
        }
    };
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

fn add_looping_menu_edges(
    menu_q: Query<Entity, Added<LoopingMenu>>,
    child_q: Query<&Children>,
    interaction_q: Query<&UiGlobalTransform, With<InteractionPalette>>,
    mut nav_map: ResMut<DirectionalNavigationMap>,
) {
    for menu_e in menu_q {
        let mut interactibles: Vec<_> = child_q
            .iter_descendants(menu_e)
            .filter_map(|e| interaction_q.get(e).ok().map(|t| (e, t.translation.y)))
            .collect();
        interactibles.sort_unstable_by_key(|(_, y)| *y as u32);
        let btm = interactibles.last().map(|(e, _)| e);
        let top = interactibles.first().map(|(e, _)| e);
        if let (Some(btm), Some(top)) = (btm, top) {
            nav_map.add_symmetrical_edge(*top, *btm, CompassOctant::North);
        }
    }
}
