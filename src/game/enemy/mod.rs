use std::collections::VecDeque;

use crate::{game::turn::TurnOrder, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .init_resource::<EnemyActionQueue>()
        .add_systems(OnEnter(TurnOrder::Ai), queue_actions)
        .add_systems(Update, process_queue.run_if(in_state(TurnOrder::Ai)))
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed);
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyKind {
    /// Short range meelee
    Chaser,
}
impl EnemyKind {
    // if some enemies can't move, then this could return Option
    pub fn tile_movement_duration_ms(&self) -> Duration {
        Duration::from_millis(match self {
            EnemyKind::Chaser => 500,
        })
    }
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub kind: EnemyKind,
}

#[derive(Resource, Deref, DerefMut, Debug, Default)]
pub struct Enemies(Vec<Entity>);

#[derive(Debug, Clone, Copy)]
pub struct EnemyAction {
    kind: EnemyActionKind,
    enemy_e: Entity,
}

#[derive(Debug, Clone, Copy)]
pub enum EnemyActionKind {
    /// Enemy ability - usually an attack
    Ability,
    Move,
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct EnemyActionQueue(VecDeque<EnemyAction>);

fn on_enemy_added(ev: On<Add, Enemy>, mut enemies: ResMut<Enemies>) {
    enemies.push(ev.entity);
}

fn on_enemy_removed(ev: On<Remove, Enemy>, mut enemies: ResMut<Enemies>) {
    if let Some(i) = enemies.iter().position(|e| *e == ev.entity) {
        enemies.remove(i);
    }
}

fn queue_actions(enemies: Res<Enemies>, mut action_queue: ResMut<EnemyActionQueue>) {
    // queue abilities first
    for e in &enemies.0 {
        action_queue.push_back(EnemyAction {
            kind: EnemyActionKind::Ability,
            enemy_e: *e,
        });
    }
    // queue movement
    for e in &enemies.0 {
        action_queue.push_back(EnemyAction {
            kind: EnemyActionKind::Move,
            enemy_e: *e,
        });
    }
}

fn process_queue(
    mut action_timer: Local<Timer>,
    mut action_queue: ResMut<EnemyActionQueue>,
    time: Res<Time>,
    mut turn: ResMut<NextState<TurnOrder>>,
    enemy_q: Query<&Enemy>,
) {
    action_timer.tick(time.delta());
    if action_timer.is_finished() {
        match action_queue.pop_front() {
            Some(action) => {
                let enemy = or_return!(enemy_q.get(action.enemy_e));
                let action_duration = match action.kind {
                    EnemyActionKind::Ability => {
                        tracing::warn!("doing a cool ability");
                        Duration::from_millis(100)
                    }
                    EnemyActionKind::Move => {
                        tracing::warn!("schmoving");
                        enemy.kind.tile_movement_duration_ms()
                    }
                };
                action_timer.set_duration(action_duration);
            }
            None => {
                turn.set(TurnOrder::Player);
            }
        }
    }
}
