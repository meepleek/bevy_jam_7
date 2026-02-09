use std::collections::VecDeque;

use crate::game::die::Die;
use crate::prelude::tween::{DespawnOnTweenCompleted, get_relative_scale_anim};
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_action)
        .add_systems(Update, kill_dice)
        .add_observer(pip_change_action);
}

#[allow(dead_code)]
#[derive(Component)]
pub struct ActionQueue {
    // todo:
    actions: VecDeque<()>,
}

#[derive(Event, Debug)]
pub struct MoveAction {
    pub agent_e: Entity,
    pub to: Coords,
    pub pip_cost: u8,
}

fn move_action(trig: On<MoveAction>, mut cmd: Commands, mut grid: Single<&mut Grid>) {
    let pos = or_return!(grid.tile_to_world(trig.to));
    or_return!(grid.move_entity(trig.agent_e, trig.to));
    or_return!(cmd.get_entity(trig.agent_e)).insert(tween::get_relative_translation_anim(
        pos,
        300,
        Some(EaseFunction::BackIn),
    ));
    if trig.pip_cost != 0 {
        cmd.trigger(PipChangeAction {
            agent_e: trig.agent_e,
            change: PipChangeKind::Offset(-(trig.pip_cost as i8)),
        });
    }
}

#[derive(Debug, PartialEq)]
pub enum PipChangeKind {
    Offset(i8),
    Reroll,
}

#[derive(Event, Debug)]
pub struct PipChangeAction {
    pub agent_e: Entity,
    pub change: PipChangeKind,
}

fn pip_change_action(trig: On<PipChangeAction>, mut die_q: Query<&mut Die>) {
    let mut die = or_return_quiet!(die_q.get_mut(trig.agent_e));
    die.pip_count = match trig.change {
        PipChangeKind::Offset(offset) => die
            .pip_count
            .saturating_add_signed(offset)
            .min(die.kind.max_pips()),
        // todo: if it's a player shift the distribution slightly in their favour
        PipChangeKind::Reroll => rng().random_range(1..=die.kind.max_pips()),
    };
}

fn kill_dice(die_q: Query<(Entity, &Die), Changed<Die>>, mut cmd: Commands) {
    for (e, _die) in die_q.iter().filter(|(_, pips)| pips.pip_count == 0) {
        or_continue!(cmd.get_entity(e)).try_insert((
            get_relative_scale_anim(Vec2::ZERO, 200, None),
            DespawnOnTweenCompleted::Itself,
        ));
    }
}
