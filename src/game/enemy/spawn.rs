use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed);
}

#[derive(Resource, Deref, DerefMut, Debug, Default)]
pub struct Enemies(pub(super) Vec<Entity>);

fn on_enemy_added(ev: On<Add, Enemy>, mut enemies: ResMut<Enemies>) {
    enemies.push(ev.entity);
}

fn on_enemy_removed(ev: On<Remove, Enemy>, mut enemies: ResMut<Enemies>) {
    if let Some(i) = enemies.iter().position(|e| *e == ev.entity) {
        enemies.remove(i);
    }
}
