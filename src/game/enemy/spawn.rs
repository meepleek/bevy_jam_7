use crate::{
    game::{heat::Heat, turn::TurnOrder},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Enemies>()
        .init_resource::<EnemyStats>()
        .add_observer(on_enemy_added)
        .add_observer(on_enemy_removed)
        .add_systems(OnEnter(Gameplay), reinit_resource::<EnemyStats>)
        .add_systems(OnEnter(TurnOrder::Player), on_player_turn);
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

fn on_enemy_removed(
    ev: On<Remove, Enemy>,
    mut enemies: ResMut<Enemies>,
    mut stats: ResMut<EnemyStats>,
    heat: Res<Heat>,
) {
    if let Some(i) = enemies.iter().position(|e| *e == ev.entity) {
        enemies.remove(i);

        stats.kill_count += 1;
        if stats.kill_count >= heat.target_kill_count() {
            tracing::warn!("shop pls!");
        }
    }
}

fn on_player_turn(
    mut cmd: Commands,
    enemies: ResMut<Enemies>,
    heat: Res<Heat>,
    sprites: Res<Sprites>,
    grid: Single<&Grid>,
) {
    if enemies.len() < heat.enemy_max() {
        // spawn a single enemy per turn (max spawns could also be based on heat)
        let player_tile = or_return!(grid.get_player_tile());
        let (_, enemy) = or_return!(random_enemy(&sprites, &grid, &[], player_tile, 0));
        cmd.spawn(enemy);
    }
}
