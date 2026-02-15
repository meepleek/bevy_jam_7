use bevy::{ecs::system::IntoObserverSystem, ui::InteractionDisabled};

use crate::{game::turn::TurnOrder, prelude::*, utils::bundle_effect::BundleEffect};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TurnOrder::Ai), disable_end_turn_button)
        .add_systems(
            Update,
            handle_temp_change.run_if(resource_exists_and_changed::<Temp>),
        )
        .add_systems(Update, tick_btns.run_if(in_state(TurnOrder::Player)));
}

#[derive(Component)]
struct EndTurnButton;

#[derive(Component)]
struct TempFill;

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

fn handle_temp_change(mut cmd: Commands, temp: Res<Temp>, fill_e: Single<Entity, With<TempFill>>) {
    // node.height = TempFill::perc_val(temp.current, temp.max);
    or_return!(cmd.get_entity(*fill_e)).try_insert((
        tween::get_relative_scale_anim(Vec2::new(1., temp.ratio()), 300, None),
        tween::get_relative_sprite_color_anim(temp.fill_color(), 250, None),
    ));
}

#[derive(Component, Debug, Clone)]
pub struct GameButtonPalette {
    pub idle: Color,
    pub click: Color,
    #[expect(dead_code)]
    pub border: Color,
    pub hover: Color,
}

#[derive(Component)]
pub struct HoldGameButton {
    timer: Timer,
    pressed: bool,
}
impl HoldGameButton {
    pub fn new(duration_ms: u64) -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Repeating),
            pressed: false,
        }
    }

    fn reset(&mut self) {
        self.pressed = false;
        self.timer.reset();
    }
}

#[derive(EntityEvent)]
pub struct GameButtonFired(Entity);

fn end_turn(_ev: On<GameButtonFired>, mut turn: ResMut<NextState<TurnOrder>>) {
    turn.set(TurnOrder::Ai);
}

fn disable_end_turn_button(btn: Single<Entity, With<EndTurnButton>>, mut cmd: Commands) {
    // todo: actually properly disable the button
    or_return!(cmd.get_entity(*btn)).insert(InteractionDisabled);
}

pub fn end_turn_btn(sprites: &Sprites) -> impl Bundle {
    let position = Vec3::new(450., 110., 1.);
    (
        Name::new("end_turn_btn"),
        EndTurnButton,
        press_game_btn_base(
            &sprites,
            position.with_x(800.),
            Vec2::new(200., 100.),
            COL_LIGHT,
            COL_RED,
            COL_CARD_BORDER,
            COL_CARD_BORDER_FOCUS,
            (Sprite {
                image: sprites.end_turn_icon.clone(),
                color: COL_PURPLE_DARK,
                ..default()
            },),
            end_turn,
        ),
        tween::get_relative_translation_anim(position.truncate(), 400, None),
    )
}

pub fn press_game_btn_base<M, I>(
    sprites: &Sprites,
    position: Vec3,
    size: Vec2,
    color: Color,
    click_color: Color,
    border_color: Color,
    hover_color: Color,
    content: impl Bundle,
    btn_fired_handler: I,
) -> impl Bundle
where
    I: IntoObserverSystem<GameButtonFired, (), M> + Sync,
{
    let palette = GameButtonPalette {
        idle: color,
        click: click_color,
        border: border_color,
        hover: hover_color,
    };

    (
        palette.clone(),
        Sprite {
            image: sprites.btn_inner_9slice.clone(),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(15.),
                ..default()
            }),
            custom_size: Some(size),
            color: color,
            ..default()
        },
        Pickable::default(),
        Transform::from_translation(position),
        Visibility::default(),
        children![(
            Sprite {
                image: sprites.btn_outline_9slice.clone(),
                image_mode: SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(18.),
                    ..default()
                }),
                custom_size: Some(size + Vec2::ONE * 8.),
                color: border_color,
                ..default()
            },
            Transform::from_translation(Vec3::Z * 0.1),
            children![content]
        )],
        BundleEffect::new(move |cmd_e| {
            cmd_e
                .observe(tween::tween_sprite_color_on_trigger::<Pointer<Over>, ()>(
                    palette.hover,
                ))
                .observe(handle_next_turn_pointer_hover)
                .observe(handle_next_turn_pointer_out)
                .observe(tween::tween_sprite_color_on_trigger::<Pointer<Out>, ()>(
                    palette.idle,
                ))
                .observe(tween::tween_sprite_color_on_trigger::<Pointer<Press>, ()>(
                    palette.click,
                ))
                .observe(
                    tween::tween_sprite_color_on_trigger::<Pointer<Release>, ()>(palette.hover),
                )
                .observe(map_pointer_event::<Click, _>(move |entity, _| {
                    GameButtonFired(entity)
                }))
                .observe(btn_fired_handler);
        }),
    )
}

#[expect(dead_code)]
pub fn hold_game_btn_base<M, I>(
    sprites: &Sprites,
    position: Vec3,
    size: Vec2,
    click_duration_ms: u64,
    color: Color,
    click_color: Color,
    border_color: Color,
    border_hover_color: Color,
    content: impl Bundle,
    btn_fired_handler: I,
) -> impl Bundle
where
    I: IntoObserverSystem<GameButtonFired, (), M> + Sync,
{
    let palette = GameButtonPalette {
        idle: color,
        click: click_color,
        border: border_color,
        hover: border_hover_color,
    };

    (
        HoldGameButton::new(click_duration_ms),
        palette,
        Sprite {
            image: sprites.btn_inner_9slice.clone(),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(15.),
                ..default()
            }),
            custom_size: Some(size),
            color: color,
            ..default()
        },
        Pickable::default(),
        Transform::from_translation(position),
        Visibility::default(),
        children![(
            Sprite {
                image: sprites.btn_outline_9slice.clone(),
                image_mode: SpriteImageMode::Sliced(TextureSlicer {
                    border: BorderRect::all(18.),
                    ..default()
                }),
                custom_size: Some(size + Vec2::ONE * 8.),
                color: border_color,
                ..default()
            },
            Transform::from_translation(Vec3::Z * 0.1),
            children![content]
        )],
        BundleEffect::new(move |cmd_e| {
            cmd_e
                .observe(on_pointer_press)
                .observe(set_btn_state::<Out>)
                .observe(set_btn_state::<Release>)
                .observe(btn_fired_handler);
        }),
    )
}

fn on_pointer_press(ev: On<Pointer<Press>>, mut btn_q: Query<&mut HoldGameButton>) {
    let mut btn = or_return!(btn_q.get_mut(ev.event_target()));
    btn.pressed = true;
}

fn set_btn_state<TPointerEvent: core::fmt::Debug + Clone + Reflect>(
    ev: On<Pointer<TPointerEvent>>,
    mut btn_q: Query<(
        &mut HoldGameButton,
        &GameButtonPalette,
        &mut Sprite,
        &mut Transform,
    )>,
) {
    let (mut btn, palette, mut sprite, mut btn_t) = or_return!(btn_q.get_mut(ev.event_target()));
    btn.reset();
    sprite.color = palette.idle;
    btn_t.scale = Vec3::ONE;
}

fn tick_btns(
    mut cmd: Commands,
    mut btn_q: Query<(
        Entity,
        &mut HoldGameButton,
        &GameButtonPalette,
        &mut Sprite,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (e, mut btn, palette, mut sprite, mut btn_t) in &mut btn_q {
        if !btn.pressed {
            continue;
        }

        btn.timer.tick(time.delta());
        let progress = btn.timer.elapsed().as_secs_f32() / btn.timer.duration().as_secs_f32();
        sprite.color = tween::lerp_color(palette.idle, palette.click, progress);
        let eased = EaseFunction::QuadraticInOut.sample_clamped(progress);
        let scale_offset = eased * 0.1;
        btn_t.scale = Vec3::new(1. + scale_offset, 1. - scale_offset, 1.);
        if btn.timer.just_finished() {
            btn.reset();
            sprite.color = palette.idle;
            or_return!(cmd.get_entity(e)).trigger(GameButtonFired);
        }
    }
}

fn handle_next_turn_pointer_hover(
    _ev: On<Pointer<Over>>,
    mut cards: Cards,
    turn_order: Res<State<TurnOrder>>,
) {
    if *turn_order.get() == TurnOrder::Player {
        cards.hide_cards();
    }
}

fn handle_next_turn_pointer_out(
    _ev: On<Pointer<Out>>,
    mut cards: Cards,
    turn_order: Res<State<TurnOrder>>,
) {
    if *turn_order.get() == TurnOrder::Player {
        cards.show_cards();
    }
}
