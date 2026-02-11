use std::collections::VecDeque;

use bevy_tweening::Animator;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_action);
}

#[derive(Component, Debug)]
pub struct Movement {
    tile_movement_speed_ms: u64,
    tile_pause_ms: u64,
}
impl Default for Movement {
    fn default() -> Self {
        Self {
            tile_movement_speed_ms: 300,
            tile_pause_ms: 100,
        }
    }
}

#[derive(Event, Debug)]
pub struct MoveAction {
    pub agent_e: Entity,
    pub to: Coords,
}

fn move_action(
    trig: On<MoveAction>,
    mut cmd: Commands,
    mut grid: Single<&mut Grid>,
    agent_q: Query<(&GlobalTransform, &Movement)>,
) {
    or_return!(grid.move_entity(trig.agent_e, trig.to));

    // create a path to move tile by tile
    let (agent_t, agent_movement) = or_return!(agent_q.get(trig.agent_e));
    let start_world_pos = agent_t.translation();
    let start_tile = or_return!(grid.world_to_tile(start_world_pos.truncate()));
    // movement can be only ortho or diag, so signum should be ok to just get the values to 0/1 to get a direction
    let dir = (trig.to - start_tile).signum();
    if dir == Coords::ZERO {
        return;
    }

    let mut current_tile = start_tile;
    let mut world_path = VecDeque::new();
    loop {
        let from_world = or_return!(grid.tile_to_world(current_tile)).extend(start_world_pos.z);
        current_tile += dir;
        let to_world = or_return!(grid.tile_to_world(current_tile));

        world_path.push_back((from_world, to_world));
        if current_tile == trig.to {
            break;
        }
    }
    // start from the current position instead of snapping to the first tile
    let (_, first_to) = world_path.pop_front().expect("empty move path");
    let tween = world_path.into_iter().fold(
        tween::delay_tween(
            tween::get_absolute_translation_tween(
                start_world_pos,
                first_to,
                agent_movement.tile_movement_speed_ms,
                None,
            ),
            agent_movement.tile_pause_ms,
        ),
        |t, (from, to)| {
            t.then(tween::delay_tween(
                tween::get_absolute_translation_tween(
                    from,
                    to,
                    agent_movement.tile_movement_speed_ms,
                    None,
                ),
                agent_movement.tile_pause_ms,
            ))
        },
    );
    or_return!(cmd.get_entity(trig.agent_e)).insert(Animator::new(tween));
}
