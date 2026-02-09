#![allow(dead_code)]

use crate::prelude::*;

#[derive(Component)]
pub struct RemovableObserver(Entity);

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

pub fn remove_observers_for_watched_entity(
    commands: &mut Commands,
    observer_q: Query<(Entity, &Observer)>,
    entity: Entity,
) {
    for (observer_e, _) in observer_q
        .iter()
        .filter(|(_, observer)| observer.descriptor().entities().contains(&entity))
    {
        or_continue!(commands.get_entity(observer_e)).try_despawn();
    }
}
