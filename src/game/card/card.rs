use bevy::color::palettes::css::BLACK;

use crate::prelude::*;
use crate::utils::bundle_effect::BundleEffect;

pub const CARD_BORDER_COL: Srgba = GRAY_950;
pub const CARD_BORDER_COL_FOCUS: Srgba = AMBER_400;
pub const CARD_BORDER_COL_DISCARD: Srgba = RED_500;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_card_face);
}

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
}

#[derive(EntityEvent)]
#[entity_event(propagate)]
pub struct CardPointerOut(Entity);

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct CardFocused;

#[derive(Component, Clone, Copy, PartialEq, Default)]
pub struct SelectedTileTriggerCard;

relationship_1_to_1!(CardContent, CardContentRoot);
relationship_1_to_1!(CardFace, CardFaceRoot);

pub fn card(card: Card, position: Vec3, rotation: Rot2, hover_mesh: Handle<Mesh>) -> impl Bundle {
    (
        Name::new("card"),
        card,
        Transform::from_translation(position),
        Visibility::default(),
        BundleEffect::new(move |e_cmd| {
            e_cmd.with_children(|b| {
                b.spawn((
                    Name::new("card_border"),
                    Sprite::from_color(CARD_BORDER_COL, Vec2::new(160., 240.)),
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    Transform::from_rotation(Quat::from_rotation_z(rotation.as_radians())),
                    ChildRotation(b.target_entity()),
                    Visibility::default(),
                    children![(
                        Name::new("card_content"),
                        CardContent(b.target_entity()),
                        Sprite::from_color(AMBER_100, Vec2::new(150., 230.)),
                        Transform::from_xyz(0., 0., 0.05),
                    )],
                ))
                .observe(tween::tween_sprite_color_on_trigger_with::<
                    ColorCardBorder,
                    (),
                >(|ev| ev.color));

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
            });
        }),
    )
}

fn spawn_card_face(
    trig: On<Add, HandCard>,
    card_q: Query<(&Card, &RotationRoot), Without<CardFaceRoot>>,
    mut cmd: Commands,
) {
    let (card, rotation_root) = or_return!(card_q.get(trig.event_target()));
    or_return!(cmd.get_entity(rotation_root.entity())).with_children(|b| {
        b.spawn((
            Name::new("card_content"),
            Visibility::default(),
            Transform::from_translation(Vec3::Z * 0.06),
            CardFace(trig.event_target()),
        ))
        .with_children(|b| {
            b.spawn((
                Name::new("card_title"),
                Text2d::new(card.effect_trigger.title()),
                TextColor::from(BLACK),
                Transform::from_translation(Vec3::Y * 90.),
            ));

            if let Some(temp_offset) = card.effect_trigger.temp_offset() {
                b.spawn((
                    Name::new("temp_offset"),
                    Text2d::new(temp_offset.to_string()),
                    TextColor::from(if temp_offset > 0 { GREEN_400 } else { RED_400 }),
                    Transform::from_translation(Vec3::new(50., 90., 0.)),
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
                        let palette = action.tile_interaction_palette();
                        let size = 20f32;
                        // center tile
                        b.spawn((Sprite::from_color(ROSE_300, Vec2::splat(size - 3.)),));
                        for tile in action.tiles() {
                            b.spawn((
                                Sprite::from_color(palette.highlight, Vec2::splat(size - 3.)),
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
                    color: CARD_BORDER_COL_DISCARD.into(),
                }))
                .observe(map_pointer_event::<Out, _>(|entity, _| ColorCardBorder {
                    entity,
                    color: CARD_BORDER_COL_FOCUS.into(),
                }));
            }
        });
    });
}

fn handle_discard_click(
    ev: On<Pointer<Click>>,
    mut cmd: Commands,
    mut observers: Observers,
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
        color: CARD_BORDER_COL.into(),
    });
    cards.discard_card(card_e);
    observers.remove_observers_for_watched_entity(e);
}
