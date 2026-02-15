use crate::{game::card, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<TurnOrder>()
        .add_systems(
            OnEnter(TurnOrder::Player),
            (fill_hand, show_cards_on_player_turn, restore_on_player_turn),
        )
        .add_systems(OnEnter(TurnOrder::Ai), (deselect_tile_card,))
        .add_systems(OnExit(TurnOrder::Player), hide_on_player_end_turn)
        .add_systems(Update, end_turn_on_empty_hand);
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameplayPhase = GameplayPhase::Gameplay)]
pub enum TurnOrder {
    #[default]
    Player,
    Ai,
}

#[derive(Component)]
struct RestoreValueOnPlayerTurn(Vec3);

#[derive(Component)]
pub enum HideOnAiTurn {
    AbsoluteX(f32),
    #[expect(dead_code)]
    RelativeX(f32),
    #[expect(dead_code)]
    AbsoluteY(f32),
    #[expect(dead_code)]
    RelativeY(f32),
}

fn end_turn_on_empty_hand(
    card_q: Query<&CardsInHand, Changed<CardsInHand>>,
    mut turn: ResMut<NextState<TurnOrder>>,
    mut piles: Single<&mut Piles>,
) {
    if piles.first_hand {
        // avoid AI starting because the initial hand is empty
        piles.first_hand = false;
        return;
    }

    let hand = or_return_quiet!(card_q.single());
    if hand.is_empty() {
        turn.set(TurnOrder::Ai);
    }
}

fn fill_hand(
    mut piles_q: Query<
        (Entity, &mut DrawPile, &CardsInHand, &mut DiscardPile),
        Changed<CardsInHand>,
    >,
    hand_size: Single<&HandSize>,
    mut cmd: Commands,
    sprites: Res<Sprites>,
) {
    let (piles_e, mut draw_pile, hand, mut discard_pile) = or_return_quiet!(piles_q.single_mut());
    let current_hand_len = hand.len();
    let draw_count = (hand_size.as_usize() - current_hand_len).max(0);
    if draw_count == 0 {
        return;
    }

    let drap_pile_size = draw_pile.len();
    let mut cards_to_draw: Vec<_> = draw_pile.drain(0..draw_count.min(drap_pile_size)).collect();
    if cards_to_draw.len() < draw_count {
        // not enough cards in the draw pile
        // shuffle the discard, take the rest from there, then move to rest to the draw pile
        let mut new_draw: Vec<_> = discard_pile.drain(..).collect();
        let mut rng = rng();
        new_draw.shuffle(&mut rng);
        for card in new_draw {
            if cards_to_draw.len() < draw_count {
                cards_to_draw.push(card);
            } else {
                draw_pile.push(card);
            }
        }
    }
    for (i, card) in cards_to_draw.into_iter().enumerate() {
        cmd.spawn((
            HandCard(piles_e),
            card::card(card, current_hand_len + i, hand_size.as_usize(), &sprites),
        ));
    }
}

fn show_cards_on_player_turn(mut cards: Cards) {
    cards.show_cards();
}

fn deselect_tile_card(
    mut cmd: Commands,
    selected_tile_card_q: Query<Entity, With<SelectedTileTriggerCard>>,
) {
    for selected_card_e in &selected_tile_card_q {
        or_return!(cmd.get_entity(selected_card_e)).try_remove::<SelectedTileTriggerCard>();
    }
}

fn hide_on_player_end_turn(mut cmd: Commands, hide_q: Query<(Entity, &HideOnAiTurn, &Transform)>) {
    for (e, hide, hide_t) in hide_q {
        let pos = hide_t.translation;
        let new_pos = match hide {
            HideOnAiTurn::AbsoluteX(x) => pos.with_x(*x),
            HideOnAiTurn::RelativeX(x) => pos.with_x(pos.x + x),
            HideOnAiTurn::AbsoluteY(y) => pos.with_y(*y),
            HideOnAiTurn::RelativeY(y) => pos.with_y(pos.y + y),
        };
        or_continue!(cmd.get_entity(e)).try_insert((
            RestoreValueOnPlayerTurn(pos),
            tween::get_relative_translation_anim(new_pos.truncate(), 300, None),
        ));
    }
}

fn restore_on_player_turn(
    mut cmd: Commands,
    restore_q: Query<(Entity, &RestoreValueOnPlayerTurn)>,
) {
    for (e, restore) in restore_q {
        or_continue!(cmd.get_entity(e))
            .try_insert((tween::get_relative_translation_anim(
                restore.0.truncate(),
                300,
                None,
            ),))
            .try_remove::<RestoreValueOnPlayerTurn>();
    }
}
