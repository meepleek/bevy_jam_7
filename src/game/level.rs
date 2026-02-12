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
    for (i, card) in starting_debug_deck().into_iter().enumerate() {
        let i = i as i16 - 3;
        let (pos, rot) = draw_pile_card_pos_rot(&mut rng, i);
        cmd.spawn(card::card(
            card,
            pos,
            Rot2::degrees(rot),
            card_hover_mesh.clone(),
        ))
        .insert(DrawPileCard(piles_e));
    }

    let grid = Grid::new(5, 5);
    cmd.spawn((
        Player,
        Movement::default(),
        TileDirection::All,
        TileEntityKind::Player,
        tile_rect(BLUE_400),
    ));

    for (x, y) in [(2, 1), (3, 3), (0, 3)] {
        cmd.spawn(chaser_enemy(
            grid.tile_to_world(Coords::new(x, y)).unwrap().extend(0.),
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
