use crate::{prelude::*, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash)
            .continue_to_state(Screen::title())
            .load_collection::<Sprites>(),
    );
}

#[derive(AssetCollection, Resource)]
pub struct Sprites {
    #[asset(path = "images/card/outer.png")]
    pub card_outer: Handle<Image>,
    #[asset(path = "images/card/border.png")]
    pub card_border: Handle<Image>,
    #[asset(path = "images/card/inner.png")]
    pub card_inner: Handle<Image>,
}
