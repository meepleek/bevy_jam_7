pub use crate::prelude::*;

pub mod grid;
pub mod tile;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin, tile::plugin));
}
