pub use crate::prelude::*;

pub mod action;
pub mod card;
pub mod card_effect;
pub mod pile;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        pile::plugin,
        card::plugin,
        card_effect::plugin,
        action::plugin,
    ));
}
