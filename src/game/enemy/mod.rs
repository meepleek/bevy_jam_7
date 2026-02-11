use std::collections::VecDeque;

use bevy::color::palettes::css::CRIMSON;

use crate::{game::turn::TurnOrder, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .init_resource::<EnemyActionQueue>()
        .add_systems(OnEnter(TurnOrder::Ai), queue_actions)
        .add_systems(Update, process_queue.run_if(in_state(TurnOrder::Ai)))
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed);
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug, Clone, Copy)]
pub enum EnemyAbility {
    Attack { reach: EffectReach, temp_offset: i8 },
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

fn queue_actions(
    enemies: Res<Enemies>,
    mut action_queue: ResMut<EnemyActionQueue>,
    movement_q: Query<(), With<Movement>>,
) {
    // queue abilities first
    for e in &enemies.0 {
        action_queue.push_back(EnemyAction {
            kind: EnemyActionKind::Ability,
            enemy_e: *e,
        });
    }
    // queue movement
    for e in &enemies.0 {
        if movement_q.contains(*e) {
            action_queue.push_back(EnemyAction {
                kind: EnemyActionKind::Move,
                enemy_e: *e,
            });
        }
    }
}

fn process_queue(
    mut cmd: Commands,
    mut action_timer: Local<Timer>,
    mut action_queue: ResMut<EnemyActionQueue>,
    time: Res<Time>,
    mut turn: ResMut<NextState<TurnOrder>>,
    enemy_q: Query<(&GlobalTransform, Option<&TileDirection>, Option<&Movement>)>,
    grid: Single<&Grid>,
    player_t: Single<&GlobalTransform, With<Player>>,
) {
    action_timer.tick(time.delta());
    if action_timer.is_finished() {
        match action_queue.pop_front() {
            Some(action) => {
                let mut rng = rng();
                let (enemy_t, enemy_dir, enemy_movement) = or_return!(enemy_q.get(action.enemy_e));
                let action_duration = match action.kind {
                    EnemyActionKind::Ability => {
                        tracing::warn!("doing a cool ability");
                        Duration::from_millis(100)
                    }
                    EnemyActionKind::Move => {
                        let tile_dir = or_return!(enemy_dir);
                        let movement = or_return!(enemy_movement);
                        let tile = or_return!(grid.world_to_tile(enemy_t.translation().truncate()));
                        let player_tile =
                            or_return!(grid.world_to_tile(player_t.translation().truncate()));
                        let path = grid.path_next_to_target(tile, player_tile, *tile_dir, &mut rng);
                        tracing::warn!(?tile, ?player_tile, ?path);
                        if let Some(path) = path
                            && let Some(to) = path.first()
                        {
                            cmd.trigger(MoveAction {
                                agent_e: action.enemy_e,
                                to: *to,
                            });
                            // todo: might have to update this if an enemy can move multiple tiles
                            Duration::from_millis(movement.tile_movement_speed_ms)
                        } else {
                            return;
                        }
                    }
                };
                action_timer.set_duration(action_duration);
                action_timer.reset();
            }
            None => {
                turn.set(TurnOrder::Player);
            }
        }
    }
}

pub fn chaser_enemy(pos: Vec3) -> impl Bundle {
    (
        Enemy,
        Movement::default(),
        TileDirection::Orthogonal,
        TileEntityKind::Enemy,
        tile_rect(CRIMSON),
        Transform::from_translation(pos),
    )
}
