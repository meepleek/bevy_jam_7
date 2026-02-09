pub use crate::prelude::*;

pub mod card;
pub mod effect;
pub mod pile;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((pile::plugin, card::plugin, effect::plugin));
}
