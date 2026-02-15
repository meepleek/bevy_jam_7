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
    #[asset(path = "images/card/bg.png")]
    pub card_bg: Handle<Image>,
    #[asset(path = "images/card/border.png")]
    pub card_border: Handle<Image>,
    #[asset(path = "images/card/inner.png")]
    pub card_inner: Handle<Image>,
    #[asset(path = "images/card/corner.png")]
    pub card_corner: Handle<Image>,
    #[asset(path = "images/card/temp_up.png")]
    pub card_temp_up: Handle<Image>,
    #[asset(path = "images/card/temp_down.png")]
    pub card_temp_down: Handle<Image>,
    // tiles
    #[asset(path = "images/tile/tile_inner.png")]
    pub tile_inner: Handle<Image>,
    #[asset(path = "images/tile/tile_outline.png")]
    pub tile_outline: Handle<Image>,
    // effects
    #[asset(path = "images/effect/move.png")]
    pub effect_move: Handle<Image>,
    #[asset(path = "images/effect/attack.png")]
    pub effect_attack: Handle<Image>,
    // bg
    #[asset(path = "images/bg/mountains.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub bg_mountains: Handle<Image>,
    // heat
    #[asset(path = "images/heat/heat_icon.png")]
    pub heat_icon: Handle<Image>,
    #[asset(path = "images/heat/thermo_bg.png")]
    pub thermo_bg: Handle<Image>,
    #[asset(path = "images/heat/thermo_btm.png")]
    pub thermo_btm: Handle<Image>,
    #[asset(path = "images/heat/thermo_notch.png")]
    pub thermo_notch: Handle<Image>,
    #[asset(path = "images/heat/thermo_outline.png")]
    pub thermo_outline: Handle<Image>,
    // enemy
    #[asset(texture_atlas_layout(tile_size_x = 79, tile_size_y = 79, columns = 23, rows = 1))]
    pub enemy_atlas_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/enemy/spritesheet.png")]
    pub enemy_sheet: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 79, tile_size_y = 79, columns = 8, rows = 1))]
    pub faces_atlas_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/enemy/faces.png")]
    pub faces_sheet: Handle<Image>,
}
