use std::num::NonZero;
use std::num::NonZeroU8;

use bevy::time::common_conditions::repeating_after_delay;
use bevy_tweening::Animator;
use bevy_tweening::BoxedTweenable;
use bevy_tweening::Sequence;
use bevy_tweening::Tracks;
use tiny_bail::or_continue;
use tiny_bail::or_return;
use tiny_bail::or_return_quiet;

use crate::prelude::tween::PriorityTween;
use crate::prelude::*;

relationship_1_to_n!(DrawPileCard, DrawPile);
relationship_1_to_n!(HandCard, CardsInHand);
relationship_1_to_n!(DiscardPileCard, DiscardPile);

const FOCUSED_CARD_Y: f32 = -220.;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(card_added_to_draw)
        .add_observer(card_added_to_hand)
        .add_observer(card_added_to_discard)
        .add_observer(restore_empty_piles::<DrawPile>)
        .add_observer(restore_empty_piles::<CardsInHand>)
        .add_observer(restore_empty_piles::<DiscardPile>)
        .add_observer(ensure_single_at_most::<SelectedTileTriggerCard>)
        .add_observer(ensure_single_at_most::<CardFocused>);
    app.add_systems(
        Update,
        reposition_hand_cards.run_if(repeating_after_delay(Duration::from_millis(300))),
    );
    app.register_type::<DrawPile>()
        .register_type::<CardsInHand>()
        .register_type::<DiscardPile>()
        .register_type::<CardFaceRoot>()
        .register_type::<CardFace>();
}

// todo: consider rewriting this so that
// piles is a singleton component (ensure_one) that tracks all the piles like draw, hand, discard using observers
// then add an create animator fn to either CardState or the Piles component
// that animates the position, rotation, scale (focus pop) & colors
// and run it from a single system
// consider making the card pile an enum component
// and keeping CardSelected & CardHovered as singleton components
#[derive(Component)]
#[require(DrawPile, CardsInHand, DiscardPile)]
pub struct Piles;

#[derive(Component, Debug, Deref)]
pub struct HandSize(pub NonZeroU8);
impl HandSize {
    pub fn as_usize(&self) -> usize {
        self.0.get() as usize
    }
}

impl Default for HandSize {
    fn default() -> Self {
        Self(NonZero::new(5).unwrap())
    }
}

fn pile_card_pos_rot(
    rng: &mut ThreadRng,
    y_sign: f32,
    card_pile_order: i16,
    translation_max_offset: f32,
    rotation_max_degree: f32,
) -> (Vec3, f32) {
    let angle = -90. + rng.random_range(-rotation_max_degree..rotation_max_degree);
    let offset_range = -translation_max_offset..translation_max_offset;
    (
        Vec3::new(
            -480. + rng.random_range(offset_range.clone()),
            -230. * y_sign + rng.random_range(offset_range),
            card_pile_order as f32 * 0.1,
        ),
        angle.to_radians(),
    )
}

pub fn draw_pile_card_pos_rot(rng: &mut ThreadRng, card_pile_order: i16) -> (Vec3, f32) {
    pile_card_pos_rot(rng, 1., card_pile_order, 10., 8.)
}

fn discard_pile_card_pos_rot(rng: &mut ThreadRng, card_pile_order: i16) -> (Vec3, f32) {
    pile_card_pos_rot(rng, -1., card_pile_order, 20., 25.)
}

fn card_added_to_draw(
    trig: On<Add, DrawPileCard>,
    mut cmd: Commands,
    card_rot_q: Query<(&RotationRoot, &CardFaceRoot)>,
    draw_pile: Single<&DrawPile>,
) {
    let mut rng = rng();
    let anim_dur_ms = 300;
    let (new_pos, new_angle) = draw_pile_card_pos_rot(&mut rng, draw_pile.len() as i16);

    or_return!(cmd.get_entity(trig.event_target())).insert(
        tween::get_relative_translation_3d_anim(new_pos, anim_dur_ms, None),
    );
    let (rotation_e, face_root) = or_return!(card_rot_q.get(trig.event_target()));
    or_return!(cmd.get_entity(rotation_e.entity())).try_insert(
        tween::get_relative_z_rotation_anim(new_angle, anim_dur_ms, None),
    );
    or_return!(cmd.get_entity(face_root.entity())).try_despawn();
}

fn card_index_mult(card_index: usize, pile_size: usize) -> f32 {
    card_index as f32 - (pile_size / 2) as f32
}

fn card_index_from_slice(entities: &[Entity], entity: Entity) -> usize {
    entities.iter().position(|e| *e == entity).unwrap_or(0)
}

fn hand_card_pos(card_index: usize, pile_size: usize) -> Vec3 {
    let pos_mult = card_index_mult(card_index, pile_size);
    Vec3::new(
        pos_mult * 150.,
        -290. - pos_mult.abs() * 25.,
        pos_mult / 10. + 1.,
    )
}

pub fn hand_card_pos_with_offset(card_index: usize, pile_size: usize, rng: &mut ThreadRng) -> Vec3 {
    let max_offset = 10f32;
    hand_card_pos(card_index, pile_size)
        + Vec3::new(
            rng.random_range(-max_offset..max_offset),
            rng.random_range(-max_offset..max_offset),
            0.,
        )
}

pub fn hand_card_rot_with_offset(card_index: usize, pile_size: usize, rng: &mut ThreadRng) -> f32 {
    let max_rot_offset = 5f32;
    hand_card_rot(card_index, pile_size)
        + rng
            .random_range(-max_rot_offset..max_rot_offset)
            .to_radians()
}

fn hand_card_rot(card_index: usize, pile_size: usize) -> f32 {
    let i = card_index_mult(card_index, pile_size);
    (-10. * i).to_radians()
}

fn card_added_to_hand(
    trig: On<Add, HandCard>,
    mut cmd: Commands,
    hand: Single<&CardsInHand>,
    rotation_q: Query<&RotationRoot>,
) {
    tracing::debug!("card added to hand");
    or_return!(cmd.get_entity(trig.event_target()))
        .observe(insert_default_on_event::<Pointer<Over>, (), CardFocused>)
        .observe(remove_on_event::<CardPointerOut, (), CardFocused>)
        .observe(on_card_click)
        .observe(move_focused_card)
        .observe(rotate_focused_card)
        .observe(rotate_unfocused_card)
        .observe(move_unfocused_card)
        .observe(tween::tween_related_sprite_color_on_trigger::<
            Add,
            CardFocused,
            RotationRoot,
        >(CARD_BORDER_COL_FOCUS))
        .observe(tween::tween_related_sprite_color_on_trigger::<
            Remove,
            CardFocused,
            RotationRoot,
        >(CARD_BORDER_COL))
        .observe(move_selected_card)
        .observe(move_deselected_card);

    let animation_duration = 300;
    let mut rng = rng();
    let mut whole_hand = hand.entities().to_vec();
    // new card is not in the target pile yet
    whole_hand.push(trig.event_target());
    for (i, e) in whole_hand.iter().enumerate() {
        let pos = hand_card_pos_with_offset(i, whole_hand.len(), &mut rng);
        let rot = hand_card_rot_with_offset(i, whole_hand.len(), &mut rng);
        or_continue!(cmd.get_entity(*e)).try_insert(tween::get_relative_translation_3d_anim(
            pos,
            animation_duration,
            Some(EaseFunction::BackOut),
        ));
        let rot_e = or_continue!(rotation_q.get(*e)).entity();
        or_continue!(cmd.get_entity(rot_e)).try_insert(tween::get_relative_z_rotation_anim(
            rot,
            animation_duration,
            None,
        ));
    }
}

fn restore_empty_piles<T: RelationshipTarget>(trig: On<Remove, T>, mut cmd: Commands) {
    // reinsert piles to retrigger adding missing empty piles
    or_return!(cmd.get_entity(trig.event_target())).insert(Piles);
}

fn reposition_hand_cards(
    piles_q: Query<&CardsInHand, Changed<CardsInHand>>,
    mut cmd: Commands,
    rotation_q: Query<&RotationRoot, Without<SelectedTileTriggerCard>>,
) {
    let hand = or_return_quiet!(piles_q.single());
    if !hand.is_empty() {
        // hand cards have changed => just tween their positions
        let anim_dur_ms = 200;
        let mut rng = rng();
        for (i, e) in hand.entities().iter().enumerate() {
            let pos = hand_card_pos_with_offset(i, hand.len(), &mut rng);
            let rot = hand_card_rot_with_offset(i, hand.len(), &mut rng);
            or_return_quiet!(cmd.get_entity(*e)).insert(tween::get_relative_translation_anim(
                pos.truncate(),
                anim_dur_ms,
                None,
            ));
            let rotation_root = or_return_quiet!(rotation_q.get(*e));
            or_return_quiet!(cmd.get_entity(rotation_root.entity()))
                .insert(tween::get_relative_z_rotation_anim(rot, anim_dur_ms, None));
        }
    }
}

fn card_added_to_discard(
    trig: On<Add, DiscardPileCard>,
    mut cmd: Commands,
    observer_q: Query<(Entity, &Observer)>,
    card_rot_q: Query<&RotationRoot>,
    discard: Single<&DiscardPile>,
) {
    let anim_dur_ms = 300;
    let mut rng = rng();
    let (new_pos, new_rot) = discard_pile_card_pos_rot(&mut rng, discard.len() as i16);
    or_return!(cmd.get_entity(trig.event_target())).insert(
        tween::get_relative_translation_3d_anim(new_pos, anim_dur_ms, None),
    );
    let rotation_e = or_return!(card_rot_q.get(trig.event_target())).entity();
    or_return!(cmd.get_entity(rotation_e)).try_insert(tween::get_relative_z_rotation_anim(
        new_rot,
        anim_dur_ms,
        None,
    ));
    remove_observers_for_watched_entity(&mut cmd, observer_q, trig.event_target());
}

fn on_card_click(
    trig: On<Pointer<Click>>,
    mut cmd: Commands,
    card_selected_q: Query<(&Card, Has<SelectedTileTriggerCard>)>,
) {
    let (card, selected) = or_return!(card_selected_q.get(trig.event_target()));
    match card.trigger {
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

fn rotate_focused_card(
    trig: On<Add, CardFocused>,
    mut cmd: Commands,
    card_q: Query<&RotationRoot, Without<SelectedTileTriggerCard>>,
) {
    let rotation_root = or_return_quiet!(card_q.get(trig.event_target()));
    or_return_quiet!(cmd.get_entity(rotation_root.entity()))
        .insert(tween::get_relative_z_rotation_anim(0., 250, None));
}

fn rotate_unfocused_card(
    trig: On<Remove, CardFocused>,
    mut cmd: Commands,
    card_q: Query<&RotationRoot, Without<SelectedTileTriggerCard>>,
    hand: Single<&CardsInHand>,
) {
    let rotation_root = or_return_quiet!(card_q.get(trig.event_target()));
    let i = card_index_from_slice(hand.entities(), trig.event_target());
    let rot = hand_card_rot_with_offset(i, hand.len(), &mut rng());

    or_return_quiet!(cmd.get_entity(rotation_root.entity()))
        .insert(tween::get_relative_z_rotation_anim(rot, 250, None));
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
    let pos = hand_card_pos_with_offset(i, hand.len(), &mut rng());

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
