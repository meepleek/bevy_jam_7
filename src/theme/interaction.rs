use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource,
    audio::sound_effect,
    input::focus::{FocusInteraction, UiFocus},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_ui_focus_change.run_if(resource_exists_and_changed::<UiFocus>),
    );
    app.load_resource::<InteractionAssets>();
    app.add_observer(play_sound_effect_on_click);
    app.add_observer(play_sound_effect_on_over);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub pressed: Color,
    pub focused: Color,
}
impl InteractionPalette {
    pub fn focus_interaction_color(&self, interaction: FocusInteraction) -> Color {
        match interaction {
            FocusInteraction::Focus => self.focused,
            FocusInteraction::Click => self.pressed,
        }
    }
}

fn handle_ui_focus_change(
    focus: Res<UiFocus>,
    mut palette_q: Query<(&InteractionPalette, &mut BackgroundColor)>,
) {
    if let Some(focus_state) = focus.focus()
        && let Ok((palette, mut bg)) = palette_q.get_mut(focus_state.entity)
    {
        *bg = palette
            .focus_interaction_color(focus_state.interaction)
            .into();
    }

    if let Some(previous_e) = focus.previous()
        && let Ok((palette, mut bg)) = palette_q.get_mut(previous_e)
    {
        *bg = palette.none.into();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            click: assets.load("audio/sound_effects/button_click.ogg"),
        }
    }
}

fn play_sound_effect_on_click(
    _: On<Pointer<Click>>,
    interaction_assets: If<Res<InteractionAssets>>,
    mut commands: Commands,
) {
    commands.spawn(sound_effect(interaction_assets.click.clone()));
}

fn play_sound_effect_on_over(
    _: On<Pointer<Over>>,
    interaction_assets: If<Res<InteractionAssets>>,
    mut commands: Commands,
) {
    commands.spawn(sound_effect(interaction_assets.hover.clone()));
}
