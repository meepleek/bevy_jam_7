use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<PlayerInputCtx>();
}

#[derive(Component)]
pub struct PlayerInputCtx;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct MovementIntent;

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
        ]),
    )
}
