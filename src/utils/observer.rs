#![allow(dead_code)]

use crate::prelude::*;
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct Observers<'w, 's> {
    cmd: Commands<'w, 's>,
    observer_q: Query<'w, 's, (Entity, &'static Observer)>,
}
impl<'w, 's> Observers<'w, 's> {
    pub fn remove_observers_for_watched_entity(&mut self, entity: Entity) {
        for (observer_e, _) in self
            .observer_q
            .iter()
            .filter(|(_, observer)| observer.descriptor().entities().contains(&entity))
        {
            or_continue!(self.cmd.get_entity(observer_e)).try_despawn();
        }
    }
}

pub fn insert_default_on_event<E: EntityEvent, B: Bundle, C: Component + Default>(
    trig: On<E, B>,
    mut cmd: Commands,
) {
    let e = trig.event_target();
    or_return_quiet!(cmd.get_entity(e)).try_insert(C::default());
}

pub fn remove_on_event<E: EntityEvent, B: Bundle, C: Component>(trig: On<E, B>, mut cmd: Commands) {
    or_return_quiet!(cmd.get_entity(trig.event_target())).try_remove::<C>();
}

pub fn remove_on_add<TAddComponent: Component, TBundleToRemove: Bundle>(
    trig: On<Add, TAddComponent>,
    mut cmd: Commands,
) {
    or_return_quiet!(cmd.get_entity(trig.event_target())).try_remove::<TBundleToRemove>();
}

pub fn ensure_single_at_most<C: Component>(
    trig: On<Add, C>,
    mut cmd: Commands,
    query: Query<Entity, With<C>>,
) {
    let target_e = trig.event_target();
    for e in query.iter().filter(|e| *e != target_e) {
        or_continue!(cmd.get_entity(e)).try_remove::<C>();
    }
}

pub fn map_event<'t, TSourceEv: EntityEvent, TTargetEv: EntityEvent<Trigger<'t>: Default>>(
    target_fn: impl Fn(Entity, &TSourceEv) -> TTargetEv,
) -> impl FnMut(On<TSourceEv>, Commands) {
    move |ev, mut cmd| {
        tiny_bail::or_return!(cmd.get_entity(ev.event_target()))
            .trigger(|e| target_fn(e, ev.event()));
    }
}

pub fn map_pointer_event<
    TPointerEv: Debug + Clone + Reflect,
    TTargetEv: EntityEvent<Trigger<'static>: Default>,
>(
    target_fn: impl Fn(Entity, &TPointerEv) -> TTargetEv,
) -> impl FnMut(On<Pointer<TPointerEv>>, Commands) {
    move |ev, mut cmd| {
        tiny_bail::or_return!(cmd.get_entity(ev.event_target()))
            .trigger(|e| target_fn(e, ev.event()));
    }
}

pub fn stop_pointer_event_propagation<TEv: Debug + Clone + Reflect>(mut ev: On<Pointer<TEv>>) {
    ev.propagate(false);
}
