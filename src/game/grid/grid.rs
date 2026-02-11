use bevy::math::U16Vec2;
use bevy::platform::collections::HashMap;
use pathfinding::directed::dijkstra;

use crate::prelude::*;

pub const TILE_SIZE: u16 = 64;

pub const DIRS_ORTHO: [Coords; 4] = [Coords::NEG_Y, Coords::X, Coords::Y, Coords::NEG_X];
pub const DIRS_DIAG: [Coords; 4] = [
    Coords::ONE,
    Coords::new(1, -1),
    Coords::NEG_ONE,
    Coords::new(-1, 1),
];
pub const DIRS: [Coords; 8] = [
    Coords::NEG_Y,
    Coords::new(1, -1),
    Coords::X,
    Coords::ONE,
    Coords::Y,
    Coords::new(-1, 1),
    Coords::NEG_X,
    Coords::NEG_ONE,
];

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (track_position, track_tile_entities));
}

#[derive(Component)]
#[require(Transform)]
pub struct Grid {
    width: u16,
    heigth: u16,
    center_global_position: Vec2,
    occupied_tiles: HashMap<Coords, TileEntity>,
    entities: HashMap<Entity, Coords>,
}

#[derive(Debug, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum PlaceError {
    Taken,
    OutOfBounds,
}

#[derive(Debug, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum MoveError {
    Taken,
    OutOfBounds,
    EntityLookupFailed,
}
impl From<PlaceError> for MoveError {
    fn from(place_err: PlaceError) -> Self {
        match place_err {
            PlaceError::Taken => Self::Taken,
            PlaceError::OutOfBounds => Self::OutOfBounds,
        }
    }
}

impl Grid {
    pub fn new(width: u16, heigth: u16) -> Self {
        if width == 0 || heigth == 0 {
            panic!("Invalid dimension - no dimension can be 0");
        }
        Self {
            width,
            heigth,
            occupied_tiles: HashMap::default(),
            entities: HashMap::default(),
            center_global_position: Vec2::ZERO,
        }
    }

    #[allow(dead_code)]
    pub fn world_center(&self) -> Vec2 {
        self.center_global_position
    }

    pub fn grid_size(&self) -> U16Vec2 {
        (self.width, self.heigth).into()
    }

    pub fn size(&self) -> Vec2 {
        self.grid_size().as_vec2() * TILE_SIZE as f32
    }

    pub fn coords_to_tile_entity(&self, coords: Coords) -> Option<TileEntity> {
        self.occupied_tiles.get(&coords).cloned()
    }

    pub fn contains_agent(&self, coords: Coords) -> bool {
        self.coords_to_tile_entity(coords)
            .is_some_and(|tile_entity| {
                matches!(
                    tile_entity.kind,
                    TileEntityKind::Player | TileEntityKind::Enemy
                )
            })
    }

    pub fn entity_to_coords(&self, entity: Entity) -> Option<Coords> {
        self.entities.get(&entity).cloned()
    }

    pub fn world_to_tile(&self, pos: Vec2) -> Option<Coords> {
        // transform world position to board space (like screen space but in tiles)
        let half_size = self.size() / 2.;
        let x = half_size.x - self.center_global_position.x + pos.x;
        let y = half_size.y + self.center_global_position.y - pos.y;
        let pos_on_board = Vec2::new(x, y);
        let coords = (pos_on_board / TILE_SIZE as f32).floor().as_i16vec2();
        if !self.within_bounds(coords) {
            return None;
        }

        Some(coords)
    }

    pub fn tile_to_world(&self, tile: Coords) -> Option<Vec2> {
        if tile.min_element() < 0 || tile.x >= self.width as i16 || tile.y >= self.heigth as i16 {
            return None;
        }

        let half_size = self.size() / 2.;
        let half_tile = TILE_SIZE as f32 / 2.;
        let tile_world = tile.as_vec2() * TILE_SIZE as f32;
        let x = tile_world.x + self.center_global_position.x + half_tile - half_size.x;
        let y = -tile_world.y + self.center_global_position.y - half_tile + half_size.y;
        Some(Vec2::new(x, y))
    }

    pub fn can_place_at(&self, coords: Coords) -> Result<(), PlaceError> {
        if !self.within_bounds(coords) {
            return Err(PlaceError::OutOfBounds);
        } else if self.occupied_tiles.contains_key(&coords) {
            return Err(PlaceError::Taken);
        }
        Ok(())
    }

    pub fn place_entity(
        &mut self,
        tile_entity: TileEntity,
        coords: Coords,
    ) -> Result<(), PlaceError> {
        self.can_place_at(coords)?;
        self.entities.insert(tile_entity.entity, coords);
        self.occupied_tiles.insert(coords, tile_entity);

        Ok(())
    }

    pub fn move_entity(&mut self, entity: Entity, coords: Coords) -> Result<(), MoveError> {
        self.can_place_at(coords)?;
        match self.entities.get(&entity) {
            Some(prev_tile) => match self.clear_tile(*prev_tile) {
                Some(tile_entity) => self.place_entity(tile_entity, coords)?,
                None => panic!("Reverse coords lookup failed"),
            },
            None => return Err(MoveError::EntityLookupFailed),
        }
        Ok(())
    }

    fn clear_tile(&mut self, coords: Coords) -> Option<TileEntity> {
        self.occupied_tiles.remove(&coords)
    }

    pub fn within_bounds(&self, tile: Coords) -> bool {
        tile.min_element() >= 0 && tile.x < self.width as _ && tile.y < self.heigth as _
    }

    fn neighbours(
        &self,
        tile: Coords,
        allowed_occupied_tile: Option<Coords>,
        move_dir: MovementDirection,
        rng: &mut impl Rng,
    ) -> Vec<Coords> {
        let dirs: &[Coords] = match move_dir {
            MovementDirection::Orthogonal => &DIRS_ORTHO,
            MovementDirection::Diagonal => &DIRS_DIAG,
            MovementDirection::All => &DIRS,
        };
        let mut neighbours: Vec<_> = dirs
            .into_iter()
            .copied()
            .filter_map(|dir| {
                let target = tile + dir;
                if allowed_occupied_tile.is_some_and(|t| t == target) {
                    return Some(target);
                }
                self.can_place_at(target).ok().map(|_| target)
            })
            .collect();
        if !neighbours.is_empty() {
            neighbours.shuffle(rng);
        }
        neighbours
    }

    fn path_to_target(
        &self,
        start: Coords,
        end: Coords,
        move_dir: MovementDirection,
        rng: &mut impl Rng,
    ) -> Option<Vec<Coords>> {
        dijkstra::dijkstra(
            &start,
            |tile| {
                self.neighbours(*tile, Some(end), move_dir, rng)
                    .into_iter()
                    .map(|tile| (tile, 1))
            },
            |tile| *tile == end,
        )
        .map(|path| path.0.into_iter().skip(1).collect::<Vec<_>>())
        .and_then(|path| if path.len() > 0 { Some(path) } else { None })
    }

    pub fn path_next_to_target(
        &self,
        start: Coords,
        end: Coords,
        move_dir: MovementDirection,
        rng: &mut impl Rng,
    ) -> Option<Vec<Coords>> {
        self.path_to_target(start, end, move_dir, rng)
            .and_then(|mut path| {
                _ = path.pop();
                if path.len() > 0 { Some(path) } else { None }
            })
    }

    #[allow(dead_code)]
    pub fn iter_tiles(&self) -> TileIterator {
        TileIterator::from_size((self.width, self.heigth))
    }

    #[allow(dead_code)]
    pub fn ascii_debug_map(&self) -> String {
        let size = self.size();
        let mut dbg_map = String::with_capacity(size.element_product() as _);
        let x_axis = (0..self.width)
            .map(|i| (i % 10).to_string())
            .collect::<String>();
        dbg_map.push_str(&format!("  {}\n", &x_axis));
        dbg_map.push_str(" 0");
        let mut prev_y = 0;
        for tile in self.iter_tiles() {
            if tile.y != prev_y {
                prev_y = tile.y;
                dbg_map.push_str(&format!("{:2}", tile.y - 1));
                dbg_map.push('\n');
                dbg_map.push_str(&format!("{:2}", tile.y));
            }
            dbg_map.push(match self.occupied_tiles.get(&tile) {
                Some(TileEntity { kind, .. }) => match kind {
                    TileEntityKind::Player => '@',
                    TileEntityKind::Enemy => '!',
                    TileEntityKind::Wall => '#',
                },
                None => '.',
            });
        }
        dbg_map.push_str(&format!("\n  {}", &x_axis));
        dbg_map
    }
}

fn track_position(mut board_q: Query<(&mut Grid, &GlobalTransform), Changed<GlobalTransform>>) {
    for (mut board, t) in &mut board_q {
        board.center_global_position = t.translation().truncate();
    }
}

fn track_tile_entities(
    entity_q: Query<(Entity, &TileEntityKind, &GlobalTransform), Changed<GlobalTransform>>,
    mut grid: Single<&mut Grid>,
) {
    for (e, kind, t) in &entity_q {
        let tile = or_continue!(grid.world_to_tile(t.translation().truncate()));
        if grid.entities.contains_key(&e) {
            or_continue!(grid.move_entity(e, tile));
        } else {
            or_continue!(grid.place_entity(
                TileEntity {
                    entity: e,
                    kind: *kind,
                },
                tile,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;
    use crate::game::prelude::TileEntityKind;

    #[test_case(0., 0., 0., 0. => Some(Coords::ONE))]
    #[test_case(64.,-64., 0., 0. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 20., -20. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 40., -40. => Some(Coords::ONE))]
    #[test_case(64., -64., 64., 0. => Some(Coords::new(1, 0)))]
    #[test_case(0., 0., 120., 0. => None)]
    #[test_case(0., 0., -128., 0. => None)]
    #[test_case(0., 0., 0., 120. => None)]
    #[test_case(0., 0., 0., -128. => None)]
    #[traced_test]
    fn world_to_tile(map_x: f32, map_y: f32, world_x: f32, world_y: f32) -> Option<Coords> {
        let mut board = Grid::new(3, 3);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.world_to_tile(Vec2::new(world_x, world_y))
    }

    #[test_case(0., 0., 0, 0 => Some(Vec2::new(-64., 64.)))]
    #[test_case(0., 0., 1, 1 => Some(Vec2::new(0., 0.)))]
    // todo: fix failing test
    // #[test_case(64.,-64., 0, 0 => Some(Vec2::new(64., -64.)))]
    #[test_case(64.,-64., 2, 2 => Some(Vec2::new(128., -128.)))]
    #[test_case(0.,0., 3, 0 => None)]
    #[test_case(0.,0., 0, 3 => None)]
    #[traced_test]
    fn tile_to_world(map_x: f32, map_y: f32, tile_x: i16, tile_y: i16) -> Option<Vec2> {
        let mut board = Grid::new(3, 3);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.tile_to_world(Coords::new(tile_x, tile_y))
    }

    #[test_case(0, 0 => matches Ok(_))]
    #[test_case(3, 3 => matches Ok(_))]
    #[test_case(4, 6 => matches Ok(_))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 9 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: i16, y: i16) -> Result<(), PlaceError> {
        let board = Grid::new(6, 9);
        board.can_place_at((x, y).into())
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: Coords = (3, 3).into();
        let mut board = Grid::new(6, 6);
        board
            .place_entity(
                TileEntity {
                    kind: TileEntityKind::Player,
                    entity: Entity::PLACEHOLDER,
                },
                coords,
            )
            .expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }

    #[test_case(3, (0, 0) => true)]
    #[test_case(3, (0, 2) => true)]
    #[test_case(3, (2, 2) => true)]
    #[test_case(3, (1, 1) => true)]
    #[test_case(3, (3, 0) => false)]
    #[test_case(3, (0, 3) => false)]
    #[test_case(3, (-1, 0) => false)]
    #[test_case(3, (0, -1) => false)]
    #[traced_test]
    fn within_bounds(size: u16, tile: (i16, i16)) -> bool {
        let board = Grid::new(size, size);
        board.within_bounds(tile.into())
    }

    #[test_case((0, 2), MovementDirection::All, 0 => Some(vec![Coords::ONE, Coords::new(2, 2)]))]
    #[test_case((1, 1), MovementDirection::All, 0 => Some(vec![Coords::new(0, 1), Coords::new(1, 2), Coords::new(2, 2)]))]
    #[test_case((1, 1), MovementDirection::All, 1 => Some(vec![Coords::new(1, 0), Coords::new(2, 1), Coords::new(2, 2)]))]
    #[test_case((1, 1), MovementDirection::Orthogonal, 0 => Some(vec![Coords::new(0, 1), Coords::new(0, 2), Coords::new(1, 2), Coords::new(2, 2)]))]
    #[test_case((1, 1), MovementDirection::Orthogonal, 1 => Some(vec![Coords::new(1, 0), Coords::new(2, 0), Coords::new(2, 1), Coords::new(2, 2)]))]
    #[test_case((1, 1), MovementDirection::Diagonal, 0 => None)]
    #[traced_test]
    fn path_to_target(
        obstacle: (i16, i16),
        move_dir: MovementDirection,
        seed: u64,
    ) -> Option<Vec<Coords>> {
        let mut board = Grid::new(3, 3);
        _ = board
            .place_entity(
                TileEntity {
                    entity: Entity::PLACEHOLDER,
                    kind: TileEntityKind::Wall,
                },
                obstacle.into(),
            )
            .expect("Failed to place obstacle");

        let mut rng = StdRng::seed_from_u64(seed);
        board.path_to_target(Coords::ZERO, (2, 2).into(), move_dir, &mut rng)
    }

    #[test_case(3, Coords::ZERO, Coords::new(2, 2), Coords::ZERO, Coords::ONE, MovementDirection::Orthogonal, 0 => Some(
        vec![
            Coords::new(0, 1),
            Coords::new(0, 2),
            Coords::new(1, 2),
        ]))]
    #[test_case(5, Coords::new(0, 3), Coords::new(2, 2), Coords::new(2, 2), Coords::new(0, 3), MovementDirection::Orthogonal, 0 => Some(
        vec![
            Coords::new(1, 3),
            Coords::new(1, 2),
        ]))]
    #[test_case(5, Coords::new(3, 4), Coords::new(3, 2), Coords::new(3, 2), Coords::new(3, 4), MovementDirection::Orthogonal, 0 => Some(
        vec![Coords::new(3, 3)]))]
    #[traced_test]
    fn path_next_to_target(
        size: u16,
        start: impl Into<Coords>,
        target: impl Into<Coords>,
        player: impl Into<Coords>,
        enemy: impl Into<Coords>,
        move_dir: MovementDirection,
        seed: u64,
    ) -> Option<Vec<Coords>> {
        let mut board = Grid::new(size, size);
        _ = board
            .place_entity(
                TileEntity {
                    entity: Entity::PLACEHOLDER,
                    kind: TileEntityKind::Player,
                },
                player.into(),
            )
            .expect("Failed to place player");
        _ = board
            .place_entity(
                TileEntity {
                    entity: Entity::PLACEHOLDER,
                    kind: TileEntityKind::Enemy,
                },
                enemy.into(),
            )
            .expect("Failed to place obstacle");

        let mut rng = StdRng::seed_from_u64(seed);
        board.path_next_to_target(start.into(), target.into(), move_dir, &mut rng)
    }
}
