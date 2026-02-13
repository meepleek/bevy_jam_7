use crate::{prelude::*, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash)
            .continue_to_state(Screen::title())
            .load_collection::<Sprites>(),
    )
    .add_systems(Startup, spawn_helper_meshes);
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
}

#[derive(Resource)]
pub struct PickingMeshes {
    pub card_hover: Handle<Mesh>,
}

fn spawn_helper_meshes(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    cmd.insert_resource(PickingMeshes {
        card_hover: meshes.add(Rectangle::new(230., 570.)),
    });
}
