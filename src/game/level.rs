use bevy::{color::palettes::tailwind::BLUE_400, time::common_conditions::once_after_delay};
use bevy_tweening::Animator;

use crate::{game::ui::ui_bundle, prelude::*};

const GRID_SIZE: u16 = 7;
const TILE_TWEEN_DURATION_MS: u64 = 280;
const TILE_TWEEN_STAGGER_MS: u64 = 60;

pub(super) fn plugin(app: &mut App) {
    let initial_delay_ms = 400;
    let grid_tweening_ms =
        (GRID_SIZE * GRID_SIZE) as u64 * TILE_TWEEN_STAGGER_MS + TILE_TWEEN_DURATION_MS;
    let grid_delay = initial_delay_ms;
    let ui_delay = initial_delay_ms + grid_tweening_ms / 2; // start mid grid tweening
    let enemies_delay = grid_delay + grid_tweening_ms + 200;
    let player_delay = enemies_delay + 1_000;
    let decks_delay = player_delay + 1_000;

    app.add_systems(
        Update,
        spawn_grid.run_if(
            in_state(GameplayPhase::LevelSpawn)
                .and(once_after_delay(Duration::from_millis(grid_delay))),
        ),
    )
    .add_systems(
        Update,
        spawn_ui.run_if(
            in_state(GameplayPhase::LevelSpawn)
                .and(once_after_delay(Duration::from_millis(ui_delay))),
        ),
    )
    .add_systems(
        Update,
        spawn_enemies.run_if(
            in_state(GameplayPhase::LevelSpawn)
                .and(once_after_delay(Duration::from_millis(enemies_delay))),
        ),
    )
    .add_systems(
        Update,
        spawn_player.run_if(
            in_state(GameplayPhase::LevelSpawn)
                .and(once_after_delay(Duration::from_millis(player_delay))),
        ),
    )
    .add_systems(
        Update,
        spawn_decks.run_if(
            in_state(GameplayPhase::LevelSpawn)
                .and(once_after_delay(Duration::from_millis(decks_delay))),
        ),
    );
}

pub fn spawn_grid(mut cmd: Commands, sprites: Res<Sprites>) {
    let grid = Grid::new(GRID_SIZE, GRID_SIZE);
    cmd.spawn((
        Name::new("grid"),
        Transform::from_translation(Vec3::Y * 105.),
        Visibility::default(),
    ))
    .with_children(|b| {
        for (i, tile) in grid.iter_tiles().enumerate() {
            let tile_pos = grid
                .tile_to_world(tile)
                .expect("valid tile world position")
                .extend(0.1);
            let tile_tween_scale = Vec2::ZERO.extend(1.);
            b.spawn((
                Name::new("grid_tile"),
                TileCoords(tile),
                Transform::from_translation(tile_pos).with_scale(tile_tween_scale),
                Sprite {
                    image: sprites.tile_outline.clone(),
                    color: COL_TILE,
                    ..default()
                },
                Animator::new(tween::delay_tween(
                    tween::get_relative_scale_tween(
                        Vec2::ONE,
                        TILE_TWEEN_DURATION_MS,
                        Some(EaseFunction::BackOut),
                    ),
                    i as u64 * TILE_TWEEN_STAGGER_MS,
                )),
                children![(
                    Name::new("grid_tile_inner"),
                    Sprite {
                        image: sprites.tile_inner.clone(),
                        color: COL_TILE,
                        ..default()
                    },
                )],
            ));
        }
    })
    .insert(grid);

    // todo: after tweening
    // next_phase.set(LevelSpawnPhase::Enemies);
}

fn spawn_enemies(mut cmd: Commands, grid: Single<&Grid>) {
    for (x, y) in [(2, 1), (6, 3), (0, 5)] {
        cmd.spawn(chaser_enemy(
            grid.tile_to_world(Coords::new(x, y)).unwrap().extend(0.),
        ));
    }
}

fn spawn_player(mut cmd: Commands, _grid: Single<&Grid>) {
    cmd.spawn((
        Player,
        Movement::default(),
        TileDirection::All,
        TileObjectKind::Player,
        tile_rect(BLUE_400),
    ));
}

fn spawn_ui(mut cmd: Commands) {
    cmd.spawn(ui_bundle());
}

fn spawn_decks(mut cmd: Commands, mut next_phase: ResMut<NextState<GameplayPhase>>) {
    cmd.insert_resource(Temp::default());
    cmd.spawn((Name::new("Character"), HandSize::default()));
    cmd.spawn((
        Name::new("Piles"),
        Piles { first_hand: true },
        DrawPile(starting_debug_deck().into_iter().collect()),
    ));
    next_phase.set(GameplayPhase::Gameplay);
}

pub fn tile_rect(color: impl Into<Color>) -> impl Bundle {
    Sprite::from_color(color.into(), Vec2::splat(50.))
}

fn starting_debug_deck() -> Vec<Card> {
    use crate::prelude::CardEffect::*;
    use crate::prelude::TileCardEffect::*;

    vec![
        Card::from_tile_effect(Move {
            target: EffectTarget {
                reach: EffectReach::Exact(1),
                direction: EffectDirection::Orthogonal,
            },
            temp_offset: 1,
        })
        .with_cool1_discard_effect(),
        Card::from_tile_effect(Move {
            target: EffectTarget {
                reach: EffectReach::Exact(1),
                direction: EffectDirection::Orthogonal,
            },
            temp_offset: 1,
        })
        .with_cool1_discard_effect(),
        Card::from_tile_effect(Move {
            target: EffectTarget {
                reach: EffectReach::Exact(1),
                direction: EffectDirection::Diagonal,
            },
            temp_offset: 1,
        })
        .with_cool1_discard_effect(),
        Card::from_tile_effect(Move {
            target: EffectTarget {
                reach: EffectReach::Exact(2),
                direction: EffectDirection::Diagonal,
            },
            temp_offset: 1,
        })
        .with_cool1_discard_effect(),
        Card::from_tile_effect(Attack {
            target: EffectTarget {
                reach: EffectReach::Range(2),
                direction: EffectDirection::Orthogonal,
            },
            temp_offset: 3,
        })
        .with_cool1_discard_effect(),
        Card::from_tile_effect(Attack {
            target: EffectTarget {
                reach: EffectReach::Range(1),
                direction: EffectDirection::Area,
            },
            temp_offset: 2,
        })
        .with_cool1_discard_effect(),
        Card::from_card_effect(TempOffset(-2)),
        Card::from_card_effect(TempOffset(-2)),
    ]
}
