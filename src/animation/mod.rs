use crate::prelude::*;

pub mod tween;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(tween::plugin);
}
