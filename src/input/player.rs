use crate::{
    game::pause::{Gameplay, Paused, Playing},
    input::menu::MenuInputCtx,
    screens::Screen,
};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use tiny_bail::or_continue;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInputCtx>()
        .add_systems(OnEnter(Playing), toggle_input_ctx::<true, PlayerInputCtx>)
        .add_systems(OnEnter(Playing), toggle_input_ctx::<false, MenuInputCtx>)
        .add_systems(OnEnter(Paused), toggle_input_ctx::<false, PlayerInputCtx>)
        .add_systems(OnEnter(Paused), toggle_input_ctx::<true, MenuInputCtx>)
        .add_systems(OnExit(Gameplay), toggle_input_ctx::<false, PlayerInputCtx>)
        .add_systems(OnExit(Gameplay), toggle_input_ctx::<true, MenuInputCtx>)
        .add_observer(handle_pause);
}

#[derive(Component)]
pub struct PlayerInputCtx;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementIntent;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PauseAction;

pub fn player_input() -> impl Bundle {
    (
        PlayerInputCtx,
        actions!(PlayerInputCtx[
            (
                Action::<MovementIntent>::new(),
                DeadZone::default(), // Apply non-uniform normalization that works for both digital and analog inputs, otherwise diagonal movement will be faster.
                DeltaScale::default(),
                // SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
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
                Action::<PauseAction>::new(),
                bindings![KeyCode::Escape, KeyCode::KeyP, GamepadButton::Start]
            )

        ]),
    )
}

// pub(super) fn plugin(app: &mut App) {
//     app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

//     // Toggle pause on key press.
//     app.add_systems(
//         Update,
//         (
//             (pause, spawn_pause_overlay, open_pause_menu).run_if(
//                 in_state(Screen::Gameplay)
//                     .and(in_state(Menu::None))
//                     .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
//             ),
//             close_menu.run_if(
//                 in_state(Screen::Gameplay)
//                     .and(not(in_state(Menu::None)))
//                     .and(input_just_pressed(KeyCode::KeyP)),
//             ),
//         ),
//     );
//     app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
//     app.add_systems(
//         OnEnter(Menu::None),
//         unpause.run_if(in_state(Screen::Gameplay)),
//     );
// }

fn handle_pause(_: On<Complete<PauseAction>>, mut next: ResMut<NextState<Screen>>) {
    next.set(Screen::paused());
}

fn toggle_input_ctx<const ENABLED: bool, TInputCtx: Component>(
    ctx_q: Query<Entity, With<PlayerInputCtx>>,
    mut cmd: Commands,
) {
    for e in ctx_q {
        or_continue!(cmd.get_entity(e)).try_insert(if ENABLED {
            ContextActivity::<TInputCtx>::ACTIVE
        } else {
            ContextActivity::<TInputCtx>::INACTIVE
        });
    }
}
