use bevy_trauma_shake::Shakes;
use bevy_tweening::Animator;
use std::collections::VecDeque;

use crate::{game::turn::TurnOrder, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .init_resource::<EnemyActionQueue>()
        .add_systems(OnEnter(TurnOrder::Ai), queue_actions)
        .add_systems(Update, process_queue.run_if(in_state(TurnOrder::Ai)))
        .add_systems(Update, animate_enemies)
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed);
}

#[derive(Component)]
pub struct Enemy;
impl Enemy {
    fn colors() -> &'static [Color] {
        &[COL_RED_LIGHT, COL_RED, COL_PINK]
    }
}

#[derive(Component)]
struct EnemyAnimationTimer(Timer);

#[derive(Component)]
struct EnemyRandomizeColor;

#[derive(Component)]
pub struct AnimationIndeces {
    start: usize,
    end: usize,
}
impl AnimationIndeces {
    pub fn range(&self) -> std::ops::RangeInclusive<usize> {
        self.start..=self.end
    }
}

#[derive(Component, Debug, Clone)]
pub enum EnemyAbility {
    Attack {
        target: EffectTarget,
        temp_offset: i8,
        // todo: lob?
    },
}
impl EnemyAbility {
    fn action_duration_ms(&self) -> u64 {
        match self {
            EnemyAbility::Attack { .. } => 500,
        }
    }

    fn effect_target(&self) -> EffectTarget {
        match self {
            EnemyAbility::Attack { target, .. } => target.clone(),
        }
    }
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
    enemy_q: Query<(&EnemyAbility, Option<&TileDirection>, Option<&Movement>)>,
    grid: Single<&Grid>,
    player_t: Single<&GlobalTransform, With<Player>>,
    mut shake: Shakes,
) {
    action_timer.tick(time.delta());
    if action_timer.is_finished() {
        match action_queue.pop_front() {
            Some(action) => {
                let mut rng = rng();
                let enemy_tile = or_return!(grid.entity_to_coords(action.enemy_e));
                let (enemy_ability, enemy_dir, enemy_movement) =
                    or_return!(enemy_q.get(action.enemy_e));
                let action_duration = match action.kind {
                    EnemyActionKind::Ability => {
                        match enemy_ability {
                            EnemyAbility::Attack {
                                target,
                                temp_offset,
                            } => {
                                if grid.effect_tiles_contain_entity_kind(
                                    enemy_tile,
                                    target.clone(),
                                    TileObjectKind::Player,
                                ) {
                                    cmd.trigger(TempChangeAction {
                                        change: *temp_offset,
                                    });
                                    shake.add_trauma(0.3);
                                    // todo: tween enemy scale
                                }
                            }
                        };
                        Duration::from_millis(enemy_ability.action_duration_ms())
                    }
                    EnemyActionKind::Move => {
                        let tile_dir = or_return!(enemy_dir);
                        let movement = or_return!(enemy_movement);
                        let player_tile =
                            or_return!(grid.world_to_tile(player_t.translation().truncate()));
                        let path = grid.path_to_reach_effect_target(
                            enemy_tile,
                            player_tile,
                            *tile_dir,
                            enemy_ability.effect_target(),
                            &mut rng,
                        );
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

fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(
        &mut EnemyAnimationTimer,
        Has<EnemyRandomizeColor>,
        &AnimationIndeces,
        &mut Sprite,
    )>,
) {
    let mut rng = rng();
    for (mut timer, randomize_col, indeces, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.is_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = rng.random_range(indeces.range());
            }
            if randomize_col {
                sprite.color = *Enemy::colors().choose(&mut rng).expect("picked a color");
            }
        }
    }
}

pub fn chaser_enemy(sprites: &Sprites, pos: Vec3, i: usize) -> impl Bundle {
    (
        Movement::default(),
        TileDirection::Orthogonal,
        EnemyAbility::Attack {
            target: EffectTarget {
                reach: EffectReach::Exact(1),
                direction: EffectDirection::Area,
            },
            temp_offset: 2,
        },
        enemy_base(sprites, pos, i),
    )
}

fn enemy_base(sprites: &Sprites, pos: Vec3, i: usize) -> impl Bundle {
    let mut rng = rng();
    let duration_sec: f32 = 0.4;
    let elapsed = Duration::from_secs_f32(rng.random_range(0.0..duration_sec));
    let body_indeces = AnimationIndeces { start: 0, end: 22 };
    let mut body_timer =
        EnemyAnimationTimer(Timer::from_seconds(duration_sec, TimerMode::Repeating));
    body_timer.0.set_elapsed(elapsed);
    let body_start_index = rng.random_range(body_indeces.range());

    let face_indeces = AnimationIndeces { start: 0, end: 6 };
    let mut face_timer = EnemyAnimationTimer(Timer::from_seconds(
        duration_sec / 1.5, // this will make the faces out of sync with the body to make the enemies look jankier
        TimerMode::Repeating,
    ));
    face_timer.0.set_elapsed(elapsed);
    let face_start_index = rng.random_range(face_indeces.range());

    (
        Enemy,
        TileObjectKind::Enemy,
        Transform::from_translation(pos).with_scale(Vec2::ZERO.extend(1.)),
        Visibility::default(),
        Animator::new(tween::delay_tween(
            tween::get_relative_scale_tween(Vec2::ONE, 300, Some(EaseFunction::BackOut)),
            80 * i as u64,
        )),
        children![(
            Sprite {
                image: sprites.enemy_sheet.clone(),
                color: *Enemy::colors()
                    .choose(&mut rng)
                    .expect("picked an enemy color"),
                texture_atlas: Some(TextureAtlas {
                    layout: sprites.enemy_atlas_layout.clone(),
                    index: body_start_index,
                }),
                ..default()
            },
            Transform::from_xyz(-8., 0., 0.1),
            body_timer,
            body_indeces,
            EnemyRandomizeColor,
            children![(
                Sprite {
                    image: sprites.faces_sheet.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprites.faces_atlas_layout.clone(),
                        index: face_start_index,
                    }),
                    ..default()
                },
                Transform::from_scale(Vec2::splat(0.75).extend(1.))
                    .with_translation(Vec3::new(3., 3., 0.2)),
                face_timer,
                face_indeces,
            )]
        ),],
    )
}
