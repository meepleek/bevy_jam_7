//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_enhanced_input::{
    action::Action,
    actions,
    prelude::{Axial, Bindings, Cardinal, DeadZone, DeltaScale},
};

use crate::{
    asset_tracking::LoadResource,
    game::{
        animation::PlayerAnimation,
        movement::{MovementController, MovementIntent, PlayerInputCtx},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
}

/// The player character.
pub fn player(
    max_speed: f32,
    player_assets: &PlayerAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(
            player_assets.ducky.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: player_animation.get_atlas_index(),
            },
        ),
        Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        PlayerInputCtx,
        actions!(PlayerInputCtx[
            (
                Action::<MovementIntent>::new(),
                DeadZone::default(), // Apply non-uniform normalization that works for both digital and analog inputs, otherwise diagonal movement will be faster.
                DeltaScale::default(),
                // SmoothNudge::default(), // Make movement smooth and independent of the framerate. To only make it framerate-independent, use `DeltaScale`.
                Bindings::spawn((
                    // Bindings like WASD or sticks are very common,
                    // so we provide built-in `SpawnableList`s to assign all keys/axes at once.
                    Cardinal::wasd_keys(),
                    Cardinal::arrows(),
                    Cardinal::dpad(),
                    Axial::left_stick(),
                )),
            ),
        ]),
        player_animation,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
