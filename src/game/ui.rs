use bevy::ui::InteractionDisabled;

use crate::{
    game::turn::TurnOrder, input::menu::ButtonClick, prelude::*, utils::bundle_effect::BundleEffect,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TurnOrder::Ai), disable_end_turn_button)
        .add_systems(
            Update,
            handle_temp_change.run_if(resource_exists_and_changed::<Temp>),
        );
}

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct TempFill;
// impl TempFill {
//     pub fn perc_val(current: u8, max: u8) -> Val {
//         percent((100. / max as f32) * current as f32)
//     }
// }

// pub fn ui_bundle() -> impl Bundle {
//     (
//         Name::new("UI Grid"),
//         Pickable::IGNORE,
//         Node {
//             display: Display::Grid,
//             row_gap: px(30),
//             column_gap: px(30),
//             width: percent(100),
//             height: percent(100),
//             padding: UiRect::all(px(30)),
//             grid_template_rows: vec![
//                 GridTrack::min_content(),
//                 GridTrack::fr(1.0),
//                 GridTrack::min_content(),
//             ],
//             grid_template_columns: vec![
//                 GridTrack::min_content(),
//                 GridTrack::fr(1.0),
//                 GridTrack::min_content(),
//             ],
//             ..default()
//         },
//         children![
//             temp_bar(),
//             (
//                 Node {
//                     justify_self: JustifySelf::End,
//                     grid_column: GridPlacement::start_span(2, 2),
//                     ..default()
//                 },
//                 children![(widget::button("End turn", end_turn), EndTurnButton),]
//             ),
//             Node {
//                 justify_self: JustifySelf::Start,
//                 grid_column: GridPlacement::start_span(2, 2),
//                 ..default()
//             },
//             Node {
//                 justify_self: JustifySelf::Start,
//                 grid_column: GridPlacement::start_span(2, 2),
//                 ..default()
//             },
//         ],
//     )
// }

pub fn thermometer(sprites: &Sprites, temp: &Temp, heat: u8, color: Color) -> impl Bundle {
    let notch_handle = sprites.thermo_notch.clone();
    let pos = Vec3::new(-520., 105., 1.);
    let scale_y = temp.ratio();
    let max_temp = temp.max;

    (
        Name::new("thermostat"),
        Sprite::from_image(sprites.thermo_bg.clone()),
        Transform::from_translation(pos.with_x(-800.)),
        children![
            (
                Name::new("thermostat_bg"),
                Sprite {
                    image: sprites.thermo_bg.clone(),
                    color: COL_PURPLE_DARKER,

                    ..default()
                },
                Transform::from_translation(Vec3::Z * 0.1)
            ),
            (
                Name::new("thermostat_fill"),
                TempFill,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(60., 315.)),
                    ..default()
                },
                Transform::from_xyz(0., -120., 0.2).with_scale(Vec3::new(1., scale_y, 1.)),
                Anchor::BOTTOM_CENTER,
            ),
            (
                Name::new("thermostat_btm"),
                Sprite {
                    image: sprites.thermo_btm.clone(),
                    color,
                    ..default()
                },
                Transform::from_xyz(0., -160., 0.3),
            ),
            (
                Name::new("thermostat_outline"),
                Sprite {
                    image: sprites.thermo_outline.clone(),
                    color: COL_LIGHT,
                    ..default()
                },
                Transform::from_translation(Vec3::Z * 0.4)
            ),
            (
                Name::new("heat_icon"),
                Sprite {
                    image: sprites.heat_icon.clone(),
                    color: COL_LIGHT,
                    ..default()
                },
                Transform::from_xyz(12., -150., 1.).with_scale(Vec2::splat(0.42).extend(1.)),
            ),
            (
                Name::new("heat_value"),
                Text2d::new(heat.to_string()),
                TextFont::from_font_size(50.),
                TextColor::from(COL_LIGHT),
                Transform::from_xyz(-19., -152., 1.),
            ),
        ],
        (BundleEffect::new(move |e_cmd| {
            e_cmd.with_children(|b| {
                let fill_height = 285.;
                let segment_size = fill_height / (max_temp - 1) as f32;
                for i in 1..max_temp {
                    b.spawn((
                        Name::new("thermostat_notch"),
                        Sprite {
                            image: notch_handle.clone(),
                            color: COL_LIGHT,
                            ..default()
                        },
                        Transform::from_xyz(
                            -15.,
                            i as f32 * -segment_size + fill_height / 2. + 53.,
                            1.,
                        ),
                    ));
                }
            });
        })),
        tween::get_relative_translation_anim(pos.truncate(), 400, Some(EaseFunction::BackOut)),
    )
}

fn end_turn(_ev: On<ButtonClick>, mut turn: ResMut<NextState<TurnOrder>>) {
    turn.set(TurnOrder::Ai);
}

fn disable_end_turn_button(btn: Single<Entity, With<EndTurnButton>>, mut cmd: Commands) {
    // todo: actually properly disable the button
    or_return!(cmd.get_entity(*btn)).insert(InteractionDisabled);
}

fn handle_temp_change(mut cmd: Commands, temp: Res<Temp>, fill_e: Single<Entity, With<TempFill>>) {
    // node.height = TempFill::perc_val(temp.current, temp.max);
    or_return!(cmd.get_entity(*fill_e)).try_insert((
        tween::get_relative_scale_anim(Vec2::new(1., temp.ratio()), 300, None),
        tween::get_relative_sprite_color_anim(temp.fill_color(), 250, None),
    ));
}
