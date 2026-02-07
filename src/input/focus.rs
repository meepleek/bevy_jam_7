use bevy::{input_focus::InputFocus, prelude::*};

use crate::theme::prelude::InteractionPalette;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UiFocus>()
        .add_observer(handle_pointer_click)
        .add_observer(handle_pointer_over)
        .add_observer(handle_pointer_out)
        .add_systems(
            Update,
            handle_input_focus_change.run_if(resource_exists_and_changed::<InputFocus>),
        );
}

#[derive(Clone, Copy, Debug)]
pub enum FocusInteraction {
    Focus,
    Click,
}

#[derive(Clone, Copy, Debug)]
pub struct UiFocusState {
    pub entity: Entity,
    pub interaction: FocusInteraction,
}

#[derive(Resource, Clone, Copy, Default, Debug)]
pub struct UiFocus {
    previous: Option<Entity>,
    focus: Option<UiFocusState>,
}
#[allow(dead_code)]
impl UiFocus {
    pub fn focus(&self) -> Option<UiFocusState> {
        self.focus
    }

    pub fn previous(&self) -> Option<Entity> {
        self.previous
    }

    pub fn set(&mut self, entity: Entity, interaction: FocusInteraction) {
        self.clear_focus();
        self.focus = Some(UiFocusState {
            entity,
            interaction,
        })
    }

    pub fn clear_focus(&mut self) {
        self.previous = self.focus.take().map(|f| f.entity);
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

fn handle_pointer_click(
    ev: On<Pointer<Click>>,
    mut focus: ResMut<UiFocus>,
    palette_q: Query<(), With<InteractionPalette>>,
) {
    let e = ev.event_target();
    if palette_q.contains(e) {
        focus.set(e, FocusInteraction::Click);
    }
}

fn handle_pointer_over(
    ev: On<Pointer<Over>>,
    // mut focus: ResMut<UiFocus>,
    mut input_focus: ResMut<InputFocus>,
    palette_q: Query<(), With<InteractionPalette>>,
) {
    let e = ev.event_target();
    if palette_q.contains(e) {
        input_focus.set(e);
    }
}

fn handle_pointer_out(
    ev: On<Pointer<Out>>,
    mut focus: ResMut<UiFocus>,
    palette_q: Query<(), With<InteractionPalette>>,
) {
    if palette_q.contains(ev.event_target()) {
        focus.clear_focus();
    }
}

fn handle_input_focus_change(input_focus: Res<InputFocus>, mut focus: ResMut<UiFocus>) {
    match input_focus.get() {
        Some(e) => focus.set(e, FocusInteraction::Focus),
        None => focus.clear_focus(),
    }
}
