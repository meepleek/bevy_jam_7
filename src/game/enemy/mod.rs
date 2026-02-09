use crate::{game::turn::TurnOrder, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(TurnOrder::Ai), ai_placeholder);
}

fn ai_placeholder(mut turn: ResMut<NextState<TurnOrder>>) {
    tracing::warn!("Thinking very hard!");
    turn.set(TurnOrder::Player);
}
