use bevy::color::palettes::css::BLACK;
use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;

use crate::assets::Sprites;
use crate::prelude::tween::PriorityTween;
use crate::prelude::*;
use crate::utils::bundle_effect::BundleEffect;

const FOCUSED_CARD_Y: f32 = -220.;

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
}

#[derive(EntityEvent)]
#[entity_event(propagate)]
pub struct CardPointerOut(Entity);

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct SelectedTileTriggerCard;

relationship_1_to_1!(CardContent, CardContentRoot);

pub fn card(
    card: Card,
    hand_i: usize,
    hand_size: usize,
    hover_mesh: Handle<Mesh>,
    sprites: &Sprites,
) -> impl Bundle {
    let card_outer_handle = sprites.card_bg.clone();
    let card_inner_handle = sprites.card_inner.clone();
    let card_border_handle = sprites.card_border.clone();
    let card_corner_handle = sprites.card_corner.clone();
    let tile_handle = sprites.tile_inner.clone();
    let tile_outline_handle = sprites.tile_outline.clone();

    let pos = hand_card_pos(hand_i, hand_size) + Vec3::Y * -200.;

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
                    b.spawn((
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
                    >(|ev| ev.color));

                    b.spawn((
                        Name::new("card_content"),
                        CardContent(b.target_entity()),
                        Sprite {
                            image: card_inner_handle,
                            color: COL_CARD,
                            ..default()
                        },
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                        Transform::from_xyz(0., 0., 0.05),
                        children![
                            (card_face(card, card_corner_handle, tile_handle, tile_outline_handle))
                        ],
                    ));

                    b.spawn((
                        Name::new("card_hover_area"),
                        Transform::from_xyz(0., -120., 0.),
                        Mesh2d(hover_mesh),
                        Pickable {
                            should_block_lower: false,
                            is_hoverable: true,
                        },
                    ))
                    .observe(|ev: On<Pointer<Out>>, mut cmd: Commands| {
                        or_return_quiet!(cmd.get_entity(ev.event_target())).trigger(CardPointerOut);
                    })
                    .observe(|mut ev: On<Pointer<Over>>| {
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
    tile_handle: Handle<Image>,
    tile_outline: Handle<Image>,
) -> impl Bundle {
    (
        Name::new("card_face"),
        Visibility::default(),
        Transform::from_translation(Vec3::Z * 0.06),
        children![(
            Name::new("card_title"),
            Text2d::new(card.effect_trigger.title()),
            TextColor::from(BLACK),
            Transform::from_translation(Vec3::Y * 90.),
        )],
        BundleEffect::new(move |b| {
            b.with_children(|b| {
                if let Some(temp_offset) = card.effect_trigger.temp_offset() {
                    b.spawn((
                        Name::new("temp_offset"),
                        Sprite {
                            image: card_corner_handle.clone(),
                            color: COL_CARD_TEMP_COST_BG,
                            ..default()
                        },
                        Transform::from_xyz(-59., 77., 0.),
                        children![(
                            Text2d::new(temp_offset.to_string()),
                            TextColor::from(if temp_offset > 0 { GREEN_400 } else { RED_400 }),
                            // Transform::from_translation(Vec3::new(50., 90., 0.)),
                        )],
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
                            Transform::from_translation(Vec3::Y * -15.),
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
                                    color = COL_TILE_VALID;
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
                    b.spawn((
                        Name::new("discard_bg"),
                        Sprite::from_color(RED_400, Vec2::new(150., 50.)),
                        Transform::from_xyz(0., -85., 0.),
                        Pickable {
                            should_block_lower: true,
                            is_hoverable: true,
                        },
                        children![(
                            Name::new("discard"),
                            Text2d::new("discard".to_string()),
                            TextColor::from(BLACK),
                        )],
                    ))
                    .observe(stop_pointer_event_propagation::<Click>)
                    .observe(handle_discard_click)
                    .observe(map_pointer_event::<Over, _>(|entity, _| ColorCardBorder {
                        entity,
                        color: COL_CARD_BORDER_DISCARD.into(),
                    }))
                    .observe(map_pointer_event::<Out, _>(|entity, _| ColorCardBorder {
                        entity,
                        color: COL_CARD_BORDER_FOCUS.into(),
                    }));
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
    Vec3::new(pos_mult * 200., -310., pos_mult / 10. + 1.)
}
