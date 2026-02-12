use crate::prelude::{tween::DespawnOnTweenCompleted, *};
use bevy_trauma_shake::Shakes;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_temp_change);
}

#[derive(Event, Debug)]
pub struct AttackAction {
    pub tile: Coords,
}

fn handle_temp_change(
    action: On<AttackAction>,
    mut shake: Shakes,
    mut grid: Single<&mut Grid>,
    mut cmd: Commands,
) {
    let tile_e = or_return!(grid.clear_tile(action.tile));
    or_return!(cmd.get_entity(tile_e.entity)).insert((
        tween::get_relative_scale_anim(Vec2::ZERO, 300, Some(EaseFunction::QuadraticIn)),
        DespawnOnTweenCompleted::Itself,
    ));
    shake.add_trauma(0.25);
}
