use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<TurnOrder>()
        .add_sub_state::<PlayerRoundPhase>()
        .add_sub_state::<AiRoundPhase>()
        .add_systems(OnEnter(TurnOrder::Player), fill_hand);
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameplayPhase = GameplayPhase::Gameplay)]
pub enum TurnOrder {
    #[default]
    Player,
    Ai,
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(TurnOrder = TurnOrder::Player)]
pub enum PlayerRoundPhase {
    #[default]
    CardSelection,
    CardEffect,
    Cleanup,
}

#[allow(dead_code)]
#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(TurnOrder = TurnOrder::Ai)]
pub enum AiRoundPhase {
    #[default]
    Attack,
    Move,
}

fn fill_hand(
    piles_q: Query<(Entity, &DrawPile, &CardsInHand, &DiscardPile), Changed<CardsInHand>>,
    hand_size: Single<&HandSize>,
    mut cmd: Commands,
) {
    let (piles_e, draw, hand, discard) = or_return_quiet!(piles_q.single());
    let draw_count = (hand_size.as_usize() - hand.len()).max(0);
    if draw_count == 0 {
        return;
    }

    let mut cards_to_draw = Vec::new();
    for e in draw.entities().iter().rev().take(draw_count) {
        or_continue!(cmd.get_entity(*e)).try_remove::<DrawPileCard>();
        cards_to_draw.push(*e);
    }
    if cards_to_draw.len() < draw_count {
        // not enough cards in the draw pile
        // shuffle the discard, take the rest from there, then move to rest to the draw pile
        let mut new_draw: Vec<_> = discard.entities().iter().cloned().collect();
        let mut rng = rng();
        new_draw.shuffle(&mut rng);
        for e in new_draw {
            or_continue!(cmd.get_entity(e)).try_remove::<DiscardPileCard>();
            if cards_to_draw.len() < draw_count {
                cards_to_draw.push(e);
            } else {
                or_continue!(cmd.get_entity(e)).try_insert(DrawPileCard(piles_e));
            }
        }
    }
    for e in cards_to_draw {
        or_continue!(cmd.get_entity(e)).try_insert(HandCard(piles_e));
    }
}
