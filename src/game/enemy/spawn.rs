use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .init_resource::<EnemyStats>()
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed)
        .add_systems(OnEnter(Gameplay), reinit_resource::<EnemyStats>);
}

#[derive(Resource, Deref, DerefMut, Debug, Default)]
pub struct Enemies(pub(super) Vec<Entity>);

#[derive(Resource, Debug, Default)]
pub struct EnemyStats {
    kill_count: usize,
}

fn on_enemy_added(ev: On<Add, Enemy>, mut enemies: ResMut<Enemies>) {
    enemies.push(ev.entity);
}

fn on_enemy_removed(ev: On<Remove, Enemy>, mut enemies: ResMut<Enemies>) {
    if let Some(i) = enemies.iter().position(|e| *e == ev.entity) {
        enemies.remove(i);
    }
}
