use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::assets::Sprites;
use crate::prelude::tween::PriorityTween;
use crate::prelude::*;
use crate::utils::bundle_effect::BundleEffect;

pub const HIDDEN_CARD_Y_OFFSET: f32 = -250.;
pub const CARD_Y: f32 = -260.;
const FOCUSED_CARD_Y: f32 = -225.;

pub(super) fn plugin(_app: &mut App) {}

#[derive(EntityEvent, Debug, Clone)]
#[entity_event(propagate, auto_propagate)]
pub struct ColorCardBorder {
    entity: Entity,
    pub color: Color,
}

#[derive(Component, Debug, Clone)]
#[require(Transform)]
pub struct Card {
    pub effect_trigger: CardEffectTrigger,
    pub discard_trigger: Option<CardEffect>,
}
impl Card {
    pub fn from_card_effect(card_effect: CardEffect) -> Self {
        Self {
            effect_trigger: CardEffectTrigger::CardSelection(card_effect),
            discard_trigger: None,
        }
    }

    pub fn from_tile_effect(tile_card_effect: TileCardEffect) -> Self {
        Self {
            effect_trigger: CardEffectTrigger::TileSelection(tile_card_effect),
            discard_trigger: None,
        }
    }

    pub fn with_discard_effect(mut self, discard_effect: CardEffect) -> Self {
        self.discard_trigger = Some(discard_effect);
        self
    }

    pub fn with_cool1_discard_effect(self) -> Self {
        self.with_discard_effect(CardEffect::TempOffset(-1))
    }

    fn outline_color(&self) -> Color {
        match self.effect_trigger {
            CardEffectTrigger::CardSelection(_) => COL_CARD_OUTLINE_ACTION_CARD,
            CardEffectTrigger::TileSelection(_) => COL_CARD_OUTLINE_TILE_CARD,
        }
    }

    fn trigger_effect_icon(&self, sprites: &Sprites) -> Handle<Image> {
        match &self.effect_trigger {
            CardEffectTrigger::CardSelection(card_effect) => {
                Self::card_effect_icon(card_effect.clone(), sprites)
            }
            CardEffectTrigger::TileSelection(tile_card_effect) => match tile_card_effect {
                TileCardEffect::Move { .. } => sprites.effect_move.clone(),
                TileCardEffect::Attack { .. } => sprites.effect_attack.clone(),
            },
        }
    }

    fn discard_effect_icon(&self, sprites: &Sprites) -> Option<Handle<Image>> {
        self.discard_trigger
            .as_ref()
            .map(|effect| Self::card_effect_icon(effect.clone(), sprites))
    }

    fn card_effect_icon(card_effect: CardEffect, sprites: &Sprites) -> Handle<Image> {
        match card_effect {
            CardEffect::TempOffset(offset) => {
                if offset > 0 {
                    sprites.card_temp_up.clone()
                } else {
                    sprites.card_temp_down.clone()
                }
            }
        }
    }
}

#[derive(EntityEvent)]
pub struct CardPointerOut(Entity);

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct SelectedTileTriggerCard;

pub fn card(card: Card, hand_i: usize, hand_size: usize, sprites: &Sprites) -> impl Bundle {
    let card_outer_handle = sprites.card_bg.clone();
    let card_inner_handle = sprites.card_inner.clone();
    let card_border_handle = sprites.card_border.clone();
    let card_corner_handle = sprites.card_corner.clone();
    let card_temp_up_handle = sprites.card_temp_up.clone();
    let tile_handle = sprites.tile_inner.clone();
    let tile_outline_handle = sprites.tile_outline.clone();
    let discard_effect_handle = card.discard_effect_icon(&sprites);
    let trigger_effect_handle = card.trigger_effect_icon(&sprites);

    let pos = hand_card_pos(hand_i, hand_size) + Vec3::Y * HIDDEN_CARD_Y_OFFSET;

    (
        Name::new("card_outline"),
        Sprite {
            image: card_outer_handle,
            color: card.outline_color(),
            ..default()
        },
        Pickable {
            should_block_lower: false,
            is_hoverable: true,
        },
        Transform::from_translation(pos),
        card.clone(),
        BundleEffect::new(move |e_cmd| {
            e_cmd
                .with_children(|b| {
                    let parent_e = b.target_entity();

                    let border_e = b
                        .spawn((
                            Name::new("card_border"),
                            Sprite {
                                image: card_border_handle,
                                color: COL_CARD_BORDER,
                                ..default()
                            },
                            Transform::from_xyz(0., 0., 0.5),
                            ChildRotation(b.target_entity()),
                            Pickable {
                                should_block_lower: false,
                                is_hoverable: true,
                            },
                        ))
                        .observe(tween::tween_sprite_color_on_trigger_with::<
                            ColorCardBorder,
                            (),
                        >(|ev| ev.color))
                        .id();

                    b.spawn((
                        Name::new("card_content"),
                        Sprite {
                            image: card_inner_handle,
                            color: COL_CARD,
                            ..default()
                        },
                        Pickable::IGNORE,
                        Transform::from_xyz(0., 0., 0.05),
                        children![
                            (card_face(
                                card,
                                card_corner_handle,
                                card_temp_up_handle,
                                tile_handle,
                                tile_outline_handle,
                                trigger_effect_handle,
                                discard_effect_handle,
                                border_e
                            ))
                        ],
                    ));

                    b.spawn((
                        Name::new("card_hover_area"),
                        Sprite {
                            color: Color::NONE,
                            custom_size: Some(Vec2::new(230., 570.)),
                            ..default()
                        },
                        Transform::from_xyz(0., -120., 0.),
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                    ))
                    .observe(|mut ev: On<Pointer<Over>>| {
                        ev.propagate(false);
                    })
                    .observe(move |mut ev: On<Pointer<Out>>, mut cmd: Commands| {
                        or_return_quiet!(cmd.get_entity(parent_e)).trigger(CardPointerOut);
                        ev.propagate(false);
                    })
                    .observe(|mut ev: On<Pointer<Click>>| {
                        ev.propagate(false);
                    });
                })
                .observe(insert_default_on_event::<Pointer<Over>, (), CardFocused>)
                .observe(remove_on_event::<CardPointerOut, (), CardFocused>)
                .observe(on_card_click)
                .observe(move_focused_card)
                .observe(move_unfocused_card)
                .observe(tween::tween_related_sprite_color_on_trigger::<
                    Add,
                    CardFocused,
                    RotationRoot,
                >(COL_CARD_BORDER_FOCUS))
                .observe(tween::tween_related_sprite_color_on_trigger::<
                    Remove,
                    CardFocused,
                    RotationRoot,
                >(COL_CARD_BORDER))
                .observe(move_selected_card)
                .observe(move_deselected_card);
        }),
    )
}

fn card_face(
    card: Card,
    card_corner_handle: Handle<Image>,
    card_temp_up_handle: Handle<Image>,
    tile_handle: Handle<Image>,
    tile_outline: Handle<Image>,
    trigger_effect_handle: Handle<Image>,
    discard_effect_handle: Option<Handle<Image>>,
    border_e: Entity,
) -> impl Bundle {
    let effect_palette = card.effect_trigger.effect_palette();
    (
        Name::new("card_face"),
        Visibility::default(),
        Transform::from_translation(Vec3::Z * 0.06),
        children![(
            Name::new("card_effect"),
            Sprite {
                image: trigger_effect_handle,
                color: effect_palette.highlighted,
                ..default()
            },
            Pickable::IGNORE,
            Transform::from_translation(Vec3::Y * 81.),
        )],
        BundleEffect::new(move |b| {
            b.with_children(move |b| {
                if let Some(temp_offset) = card.effect_trigger.temp_offset() {
                    b.spawn((
                        Name::new("temp_offset"),
                        Sprite {
                            image: card_corner_handle.clone(),
                            color: COL_CARD_TEMP_COST_BG,
                            ..default()
                        },
                        Pickable::IGNORE,
                        Transform::from_xyz(-57., 81., 0.),
                        children![
                            (
                                Sprite {
                                    image: card_temp_up_handle.clone(),
                                    color: COL_CARD_TEMP_UP,
                                    ..default()
                                },
                                Pickable::IGNORE,
                                Transform::from_xyz(8., 0., 0.1)
                            ),
                            (
                                Text2d::new(temp_offset.abs().to_string()),
                                TextFont::from_font_size(35.),
                                TextColor::from(COL_CARD_TEMP_UP),
                                Transform::from_xyz(-14., 0., 0.)
                            )
                        ],
                    ));
                }

                match &card.effect_trigger {
                    CardEffectTrigger::CardSelection(_action) => {
                        // todo: smt for card actions
                        //  b.spawn((
                        //             Name::new("immediate_action"),
                        //             Text2d::new("Do the thing"),
                        //             TextColor::from(BLACK),
                        //         ));
                    }
                    CardEffectTrigger::TileSelection(action) => {
                        b.spawn((
                            Name::new("effect_tiles"),
                            Transform::from_translation(Vec3::Y * -23.),
                            Visibility::default(),
                        ))
                        .with_children(|b| {
                            let size = 26f32;
                            let effect_tiles = action.tiles();
                            let mut all_tiles_within_range = EffectTarget {
                                reach: EffectReach::Range(2),
                                direction: EffectDirection::Area,
                            }
                            .target_tiles();
                            all_tiles_within_range.push(Coords::ZERO);
                            for tile in all_tiles_within_range {
                                let mut color = COL_CARD_INVALID_TILE;
                                let mut image = tile_handle.clone();
                                if tile == Coords::ZERO {
                                    color = COL_CARD_CENTER_TILE;
                                } else if effect_tiles.contains(&tile) {
                                    color = effect_palette.highlighted;
                                } else {
                                    image = tile_outline.clone();
                                }
                                b.spawn((
                                    Sprite {
                                        image,
                                        color,
                                        custom_size: Some(Vec2::splat(size - 4.)),
                                        ..default()
                                    },
                                    Transform::from_translation(tile.as_vec2().extend(0.) * size),
                                ));
                            }
                        });
                    }
                }

                if card.discard_trigger.is_some() {
                    let discard_effect_handle =
                        discard_effect_handle.expect("missing discard effect icon");
                    b.spawn((
                        Name::new("discard_bg"),
                        Sprite {
                            image: card_corner_handle.clone(),
                            color: COL_CARD_BORDER_DISCARD,
                            flip_x: true,
                            ..default()
                        },
                        Pickable {
                            should_block_lower: true,
                            is_hoverable: true,
                        },
                        Transform::from_xyz(57., 81., 0.),
                        children![(
                            Sprite {
                                image: discard_effect_handle,
                                color: COL_CARD_TEMP_DOWN,
                                ..default()
                            },
                            Transform::from_translation(Vec3::Z * 0.1),
                        ),],
                    ))
                    .observe(stop_pointer_event_propagation::<Click>)
                    .observe(handle_discard_click)
                    .observe(tween::tween_sprite_color_on_trigger::<Pointer<Over>, ()>(
                        COL_CARD_BORDER_DISCARD_HOVER,
                    ))
                    .observe(tween::tween_sprite_color_on_trigger::<Pointer<Out>, ()>(
                        COL_CARD_BORDER_DISCARD,
                    ))
                    .observe(map_pointer_event::<Over, _>(move |_entity, _| {
                        ColorCardBorder {
                            entity: border_e,
                            color: COL_CARD_BORDER_DISCARD_HOVER.into(),
                        }
                    }))
                    .observe(map_pointer_event::<Out, _>(
                        move |_entity, _| ColorCardBorder {
                            entity: border_e,
                            color: COL_CARD_BORDER_FOCUS.into(),
                        },
                    ));
                }
            });
        }),
    )
}

fn handle_discard_click(
    ev: On<Pointer<Click>>,
    mut cmd: Commands,
    mut cards: Cards,
    hiearchy: Hiearchy<Card>,
) {
    let e = ev.event_target();
    let (card_e, card) = or_return!(hiearchy.get_self_or_ancestor(e));
    match or_return!(card.discard_trigger.as_ref()) {
        CardEffect::TempOffset(offset) => cmd.trigger(TempChangeAction { change: *offset }),
    };
    or_return!(cmd.get_entity(e)).trigger(|entity| ColorCardBorder {
        entity,
        color: COL_CARD_BORDER.into(),
    });
    cards.discard_card(card_e);
}

fn on_card_click(
    trig: On<Pointer<Click>>,
    mut cmd: Commands,
    card_selected_q: Query<(&Card, Has<SelectedTileTriggerCard>)>,
) {
    let (card, selected) = or_return!(card_selected_q.get(trig.event_target()));
    match card.effect_trigger {
        CardEffectTrigger::TileSelection(_) => {
            if selected {
                // deselect card
                or_return!(cmd.get_entity(trig.event_target()))
                    .try_remove::<SelectedTileTriggerCard>()
                    .try_remove::<CardFocused>();
            } else {
                // select card
                or_return!(cmd.get_entity(trig.event_target())).insert(SelectedTileTriggerCard);
            }
        }
        CardEffectTrigger::CardSelection(_) => {
            cmd.trigger(PlayCard(trig.event_target()));
        }
    }
}

fn move_focused_card(
    trig: On<Add, CardFocused>,
    mut cmd: Commands,
    card_q: Query<Has<SelectedTileTriggerCard>, Without<PriorityTween<Transform>>>,
    hand: Single<&CardsInHand>,
) {
    let card_e = trig.event_target();
    let is_selected = or_return!(card_q.get(card_e));
    if is_selected {
        return;
    }
    let i = card_index_from_slice(hand.entities(), trig.event_target());
    let pos = hand_card_pos(i, hand.len());

    or_return_quiet!(cmd.get_entity(card_e)).insert(tween::get_relative_translation_anim(
        pos.truncate().with_y(FOCUSED_CARD_Y),
        250,
        Some(EaseFunction::BackOut),
    ));
}

fn move_unfocused_card(
    trig: On<Remove, CardFocused>,
    mut cmd: Commands,
    card_q: Query<Has<SelectedTileTriggerCard>, Without<PriorityTween<Transform>>>,
    hand: Single<&CardsInHand>,
) {
    let card_e = trig.event_target();
    let is_selected = or_return_quiet!(card_q.get(card_e));
    if is_selected {
        return;
    }
    let i = card_index_from_slice(hand.entities(), trig.event_target());
    let pos = hand_card_pos(i, hand.len());

    or_return!(cmd.get_entity(card_e)).insert(Animator::new(Tracks::new([
        tween::get_relative_translation_tween(pos.truncate(), 250, Some(EaseFunction::BackOut)),
        // reset size in case other interactions overlap
        tween::get_relative_scale_tween(Vec2::splat(1.), 220, Some(EaseFunction::QuinticOut)),
    ])));
}

fn move_selected_card(trig: On<Add, SelectedTileTriggerCard>, mut cmd: Commands) {
    or_return!(cmd.get_entity(trig.event_target())).insert(Animator::new(Tracks::new([
        Box::new(tween::get_relative_translation_3d_tween(
            Vec3::new(-470., 0., 5.),
            350,
            Some(EaseFunction::BackOut),
        )) as BoxedTweenable<_>,
        tween::get_relative_scale_tween(Vec2::splat(1.15), 80, Some(EaseFunction::QuadraticOut))
            .then(tween::get_relative_scale_tween(
                Vec2::splat(1.),
                260,
                Some(EaseFunction::QuinticOut),
            ))
            .into(),
    ])));
}

fn move_deselected_card(
    trig: On<Remove, SelectedTileTriggerCard>,
    mut cmd: Commands,
    hand: Single<&CardsInHand>,
) {
    let i = card_index_from_slice(hand.entities(), trig.event_target());
    let pos = hand_card_pos(i, hand.len());

    or_return!(cmd.get_entity(trig.event_target())).insert((
        Animator::new(Tracks::new([
            tween::get_relative_translation_tween(pos.truncate(), 400, Some(EaseFunction::BackOut))
                .into(),
            Box::new(Sequence::new([tween::get_relative_scale_tween(
                Vec2::splat(0.8),
                80,
                Some(EaseFunction::QuadraticOut),
            )
            .then(tween::get_relative_scale_tween(
                Vec2::splat(1.),
                260,
                Some(EaseFunction::QuinticOut),
            ))])) as BoxedTweenable<_>,
        ])),
        PriorityTween::<Transform>::default(),
    ));
}

fn card_index_mult(card_index: usize, pile_size: usize) -> f32 {
    card_index as f32 - (pile_size / 2) as f32
}

fn card_index_from_slice(entities: &[Entity], entity: Entity) -> usize {
    entities.iter().position(|e| *e == entity).unwrap_or(0)
}

pub fn hand_card_pos(card_index: usize, current_hand_size: usize) -> Vec3 {
    let pos_mult = card_index_mult(card_index, current_hand_size);
    Vec3::new(pos_mult * 210., CARD_Y, pos_mult / 10. + 1.)
}
