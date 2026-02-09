use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_action);
}

#[derive(Event, Debug)]
pub struct MoveAction {
    pub agent_e: Entity,
    pub to: Coords,
    pub temp_offset: i8,
}

fn move_action(trig: On<MoveAction>, mut cmd: Commands, mut grid: Single<&mut Grid>) {
    let pos = or_return!(grid.tile_to_world(trig.to));
    or_return!(grid.move_entity(trig.agent_e, trig.to));
    or_return!(cmd.get_entity(trig.agent_e)).insert(tween::get_relative_translation_anim(
        pos,
        300,
        Some(EaseFunction::BackIn),
    ));
    if trig.temp_offset != 0 {
        cmd.trigger(TempChangeAction {
            change: -(trig.temp_offset as i8),
        });
    }
}
