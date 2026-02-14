use bevy::ecs::system::SystemParam;
use bevy::math::U16Vec2;

use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(hide_tile_highlights_on_card_deselected);
}

#[derive(SystemParam)]
pub struct Tiles<'w, 's> {
    cmd: Commands<'w, 's>,
    interaction_tile_q: Query<'w, 's, Entity, With<TileHighlighted>>,
    observers: Observers<'w, 's>,
}
impl<'w, 's> Tiles<'w, 's> {
    pub fn hide_tile_highlights(&mut self) {
        for e in &self.interaction_tile_q {
            or_continue!(self.cmd.get_entity(e))
                .try_insert(tween::get_relative_sprite_color_anim(COL_TILE, 150, None));
            self.observers.remove_observers_for_watched_entity(e);
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct TileCoords(pub Coords);

// use this as a single source of truth for both the movement & ability direction
// to avoid tricky combos like ortho movement + diag attack that could lead to buggy pathfinding
// this should also simplify the UI & mental overhead for players
#[derive(Component, Debug, Clone, Copy)]
pub enum TileDirection {
    Orthogonal,
    Diagonal,
    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileObject {
    pub entity: Entity,
    pub kind: TileObjectKind,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TileObjectKind {
    Player,
    Enemy,
    Wall,
}

#[derive(Component)]
pub struct TileHighlighted;

pub struct TileIterator {
    grid_size: U16Vec2,
    tile: Coords,
}
impl Iterator for TileIterator {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.grid_size.as_i16vec2();
        if self.tile.y >= size.y as i16 {
            None
        } else {
            let next = self.tile;
            self.tile.x += 1;
            if self.tile.x == size.x {
                self.tile = (0, self.tile.y + 1).into();
            }
            Some(next)
        }
    }
}
impl TileIterator {
    pub fn from_size(grid_size: impl Into<U16Vec2>) -> Self {
        Self {
            grid_size: grid_size.into(),
            tile: Coords::ZERO,
        }
    }
}

fn hide_tile_highlights_on_card_deselected(
    _ev: On<Remove, SelectedTileTriggerCard>,
    mut tiles: Tiles,
) {
    tiles.hide_tile_highlights();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let tiles: Vec<_> = TileIterator::from_size((5, 3)).collect();
        assert_eq!(
            tiles,
            [
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (3, 1),
                (4, 1),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
            ]
            .map(Into::into)
        );
    }
}
