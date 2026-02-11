use bevy::color::palettes::css::CRIMSON;

use crate::game::card;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayPhase::LevelSpawn), spawn_level);
}

fn spawn_level(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_phase: ResMut<NextState<GameplayPhase>>,
) {
    let mut rng = rng();
    cmd.insert_resource(Temp::default());
    cmd.spawn((Name::new("Character"), HandSize::default()));
    let piles_e = cmd.spawn((Name::new("Piles"), Piles)).id();
    let card_hover_mesh = meshes.add(Rectangle::new(230., 570.));
    for (i, action) in starting_debug_deck().into_iter().enumerate() {
        let i = i as i16 - 3;
        let (pos, rot) = draw_pile_card_pos_rot(&mut rng, i);
        cmd.spawn(card::card(
            action,
            pos,
            Rot2::degrees(rot),
            card_hover_mesh.clone(),
        ))
        .insert(DrawPileCard(piles_e));
    }

    let grid = Grid::new(5, 5);
    cmd.spawn((tile_rect(BLUE_400, TileEntityKind::Player), Player));

    for (x, y) in [/*(2, 1), (3, 3),*/ (0, 3)] {
        cmd.spawn((
            Enemy {
                kind: EnemyKind::Chaser,
            },
            tile_rect(CRIMSON, TileEntityKind::Enemy),
            Transform::from_translation(grid.tile_to_world(Coords::new(x, y)).unwrap().extend(0.)),
        ));
    }

    cmd.spawn((
        Name::new("grid"),
        Transform::from_translation(Vec3::NEG_Z),
        Visibility::default(),
    ))
    .with_children(|b| {
        let size = grid.grid_size();
        for tile in
            (0..size.y).flat_map(|y| (0..size.x).map(move |x| Coords::new(x as i16, y as i16)))
        {
            b.spawn((
                Name::new("grid_tile"),
                Transform::from_translation(grid.tile_to_world(tile).unwrap().extend(0.)),
                Sprite::from_color(GRAY_300, Vec2::splat(TILE_SIZE as f32 - 6.)),
            ));
        }
    });

    cmd.spawn(grid);

    next_phase.set(GameplayPhase::Gameplay);
}

fn tile_rect(color: impl Into<Color>, kind: TileEntityKind) -> impl Bundle {
    (Sprite::from_color(color.into(), Vec2::splat(50.)), kind)
}

fn starting_debug_deck() -> Vec<CardEffectTrigger> {
    use crate::prelude::CardEffect::*;
    use crate::prelude::TileCardEffect::*;

    vec![
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(2),
            direction: EffectDirection::Diagonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::TileSelection(Attack {
            reach: EffectReach::Range(2),
            direction: EffectDirection::Orthogonal,
            temp_offset: 2,
            attack: 2,
        }),
        CardEffectTrigger::TileSelection(Attack {
            reach: EffectReach::Range(1),
            direction: EffectDirection::Orthogonal,
            temp_offset: 2,
            attack: 3,
        }),
        CardEffectTrigger::TileSelection(Move {
            reach: EffectReach::Exact(2),
            direction: EffectDirection::Orthogonal,
            temp_offset: 1,
        }),
        CardEffectTrigger::CardSelection(HealSelf(2)),
    ]
}
