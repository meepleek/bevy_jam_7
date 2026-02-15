use bevy::ecs::system::SystemParam;
use bevy::time::common_conditions::repeating_after_delay;
use std::num::NonZero;
use std::num::NonZeroU8;
use tiny_bail::or_return;
use tiny_bail::or_return_quiet;

use crate::game::turn::TurnOrder;
use crate::prelude::*;

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct DrawPile(pub Vec<Card>);

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct DiscardPile(Vec<Card>);

relationship_1_to_n!(HandCard, CardsInHand);

pub(super) fn plugin(app: &mut App) {
    app.add_observer(restore_empty_piles::<CardsInHand>)
        .add_observer(ensure_single_at_most::<SelectedTileTriggerCard>)
        .add_observer(ensure_single_at_most::<CardFocused>);
    app.add_systems(
        Update,
        reposition_hand_cards.run_if(
            in_state(TurnOrder::Player).and(repeating_after_delay(Duration::from_millis(300))),
        ),
    );
    app.register_type::<CardsInHand>();
}

#[derive(SystemParam)]
pub struct Cards<'w, 's> {
    cmd: Commands<'w, 's>,
    card_q: Query<'w, 's, &'static Card>,
    selected_tile_card_q: Query<'w, 's, Entity, With<SelectedTileTriggerCard>>,
    discard_pile: Single<'w, 's, &'static mut DiscardPile>,
    #[expect(dead_code)]
    draw_pile: Single<'w, 's, Entity, With<DrawPile>>,
    hand: Single<'w, 's, &'static CardsInHand>,
    observers: Observers<'w, 's>,
    tiles: Tiles<'w, 's>,
}
impl<'w, 's> Cards<'w, 's> {
    pub fn discard_card(&mut self, card_e: Entity) {
        let card = or_return!(self.card_q.get(card_e)).clone();
        self.observers.remove_observers_for_watched_entity(card_e);
        or_return!(self.cmd.get_entity(card_e))
            .try_remove::<Card>()
            .try_remove::<HandCard>()
            .try_insert(tween::get_relative_scale_anim(
                Vec2::ZERO,
                300,
                Some(EaseFunction::QuadraticIn),
            ));
        // todo: particles
        self.discard_pile.push(card);
        // deselect any (other) selected tile cards on play of the discarded card
        for selected_card_e in &self.selected_tile_card_q {
            // don't remove for the discarded card to prevent triggering the observer watching for removal of that component that tweens it back it to place otherwise
            if selected_card_e != card_e {
                or_return!(self.cmd.get_entity(selected_card_e))
                    .try_remove::<SelectedTileTriggerCard>();
            }
        }
        self.tiles.hide_tile_highlights();
    }

    pub fn show_cards(&mut self) {
        self.tween_cards_position(0.);
    }

    pub fn hide_cards(&mut self) {
        self.tween_cards_position(HIDDEN_CARD_Y_OFFSET);
    }

    fn tween_cards_position(&mut self, card_y_offset: f32) {
        let anim_dur_ms = 200;
        if self.hand.is_empty() {
            return;
        }

        for (i, e) in self.hand.entities().iter().enumerate() {
            let mut pos = hand_card_pos(i, self.hand.len());
            pos.y += card_y_offset;
            or_return_quiet!(self.cmd.get_entity(*e)).insert(bevy_tweening::Animator::new(
                tween::delay_tween(
                    tween::get_relative_translation_tween(pos.truncate(), anim_dur_ms, None),
                    i as u64 * 70,
                ),
            ));
        }
    }
}

#[derive(Component, Default)]
#[require(DrawPile, CardsInHand, DiscardPile)]
pub struct Piles {
    pub first_hand: bool,
}

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

fn reposition_hand_cards(piles_q: Query<(), Changed<CardsInHand>>, mut cards: Cards) {
    if !piles_q.is_empty() {
        // hand cards have changed => just tween their positions
        cards.show_cards();
    }
}

fn restore_empty_piles<T: RelationshipTarget>(trig: On<Remove, T>, mut cmd: Commands) {
    // reinsert piles to retrigger adding missing empty piles
    or_return!(cmd.get_entity(trig.event_target())).insert(Piles::default());
}
