// #![allow(dead_code)]

use bevy::prelude::*;

pub mod bundle_effect;
pub mod extend;
pub mod initial;
pub mod math;
pub mod observer;
pub mod relationship;

pub fn plugin(app: &mut App) {
    app.add_plugins((bundle_effect::plugin, initial::plugin));
}

#[allow(unused_imports)]
pub mod prelude {
    pub use super::extend::prelude::*;
    pub use super::initial::Initial;
    pub use super::observer::*;
    pub use super::relationship::*;
}
