use crate::{
    game::ui::{GameButtonFired, press_game_btn_base},
    prelude::*,
    utils::{
        bundle_effect::BundleEffect,
        state::{HideOnStateChange, HideTween},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayPhase::Shop), show_shop);
}

#[derive(Component)]
struct Shop;

fn show_shop(mut cmd: Commands, sprites: Res<Sprites>, mut cards: Cards) {
    cmd.spawn(shop(&sprites, cards_bundle(&sprites)));
    cards.hide_cards();
}

pub fn shop(sprites: &Sprites, content: impl Bundle) -> impl Bundle {
    let size = Vec2::new(1000., 660.);
    let pos = Vec3::new(80., 0., 10.);
    (
        Shop,
        Sprite {
            image: sprites.btn_inner_9slice.clone(),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(15.),
                ..default()
            }),
            custom_size: Some(size),
            color: COL_PURPLE_DARKER,
            ..default()
        },
        Transform::from_translation(pos.with_y(-400.)),
        children![
            (
                Sprite {
                    image: sprites.btn_outline_9slice.clone(),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer {
                        border: BorderRect::all(18.),
                        max_corner_scale: 0.65,
                        ..default()
                    }),
                    custom_size: Some(size - Vec2::ONE * 25.),
                    color: COL_LIGHT,
                    ..default()
                },
                Transform::from_translation(Vec3::Z * 0.1),
            ),
            (content),
            (press_game_btn_base(
                &sprites,
                Vec3::new(0., -210., 0.1),
                Vec2::new(140., 100.),
                COL_LIGHT,
                COL_BLUE_DARK,
                COL_BLUE,
                COL_YELLOW,
                (
                    Transform::default(),
                    Visibility::default(),
                    children![(
                        Sprite {
                            image: sprites.end_turn_icon.clone(),
                            color: COL_BLUE,
                            ..default()
                        },
                        Transform::from_xyz(0., 0., 0.1),
                    )]
                ),
                exit_shop
            ))
        ],
        tween::get_relative_translation_anim(pos.truncate(), 500, Some(EaseFunction::BackOut)),
        HideOnStateChange::hide_on_exit(HideTween::AbsoluteY(-600.), GameplayPhase::Shop)
            .with_despawn(),
    )
}

fn exit_shop(_ev: On<GameButtonFired>, mut next_phase: ResMut<NextState<GameplayPhase>>) {
    tracing::warn!("byeee shop");
    next_phase.set(GameplayPhase::LevelSpawn);
}

fn cards_bundle(sprites: &Sprites) -> impl Bundle {
    let card_count = 4;
    let card_w = 220.;

    let cards: Vec<_> = (0..card_count)
        .map(|i| {
            let pos = Vec3::new(
                i as f32 * card_w - ((card_count as f32 * card_w) / 2.65),
                -60.,
                1.,
            );

            (
                Transform::from_translation(pos),
                Visibility::default(),
                children![
                    (
                        Sprite {
                            image: sprites.card_bg.clone(),
                            color: COL_LIGHT,
                            ..default()
                        },
                        Transform::from_xyz(0., 200., 0.1),
                    ),
                    (press_game_btn_base(
                        &sprites,
                        Vec3::ZERO,
                        Vec2::new(140., 100.),
                        COL_CARD_BORDER,
                        COL_PURPLE_DARK,
                        COL_LIGHT,
                        COL_PURPLE,
                        (
                            Transform::default(),
                            Visibility::default(),
                            children![
                                (
                                    Sprite {
                                        image: sprites.tooth_icon.clone(),
                                        color: COL_LIGHT,
                                        ..default()
                                    },
                                    Transform::from_xyz(25., 2., 0.1),
                                ),
                                (
                                    Text2d::new("-6"),
                                    TextFont::from_font_size(50.),
                                    TextColor::from(COL_LIGHT),
                                    Transform::from_xyz(-25., 0., 0.1),
                                )
                            ]
                        ),
                        |_: On<_>| {}
                    ))
                ],
            )
        })
        .collect();

    (
        Transform::from_translation(Vec3::Z),
        Visibility::default(),
        BundleEffect::new(move |e_cmd| {
            e_cmd.with_children(|b| {
                for card in cards {
                    b.spawn(card);
                }
            });
        }),
    )
}
