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
    app.add_systems(Update, track_grid_position)
        .add_systems(Last, add_new_tile_entities_to_grid);
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

    fn effect_tiles(
        &self,
        center: Coords,
        allowed_occupied_tiles: Option<Vec<Coords>>,
        effect_target: EffectTarget,
        allowed_entity_kind: Option<TileEntityKind>,
    ) -> Vec<Coords> {
        effect_target
            .target_tiles()
            .into_iter()
            .filter_map(|t| {
                let target = center + t;
                let is_effect_tile = match allowed_entity_kind {
                    Some(kind) => self
                        .occupied_tiles
                        .get(&target)
                        .is_some_and(|entity| entity.kind == kind),
                    None => self.can_place_at(target).is_ok(),
                } || (self.within_bounds(target)
                    && allowed_occupied_tiles
                        .as_ref()
                        .is_some_and(|allowed| allowed.contains(&target)));

                is_effect_tile.then_some(target)
            })
            .collect()
    }

    fn effect_tiles_with_start_tile(
        &self,
        center: Coords,
        start_tile: Coords,
        effect_target: EffectTarget,
    ) -> Vec<Coords> {
        self.effect_tiles(center, Some(vec![start_tile]), effect_target, None)
    }

    pub fn effect_tiles_contain_entity_kind(
        &self,
        center: Coords,
        effect_target: EffectTarget,
        entity_kind: TileEntityKind,
    ) -> bool {
        let effect_tiles = self.effect_tiles(center, None, effect_target, Some(entity_kind));
        !effect_tiles.is_empty()
    }

    fn neighbours(
        &self,
        tile: Coords,
        allowed_occupied_tile: Option<Coords>,
        move_dir: TileDirection,
        rng: &mut impl Rng,
    ) -> Vec<Coords> {
        let dirs: &[Coords] = match move_dir {
            TileDirection::Orthogonal => &DIRS_ORTHO,
            TileDirection::Diagonal => &DIRS_DIAG,
            TileDirection::All => &DIRS,
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

    pub fn path_to_reach_effect_target(
        &self,
        start: Coords,
        target_tile: Coords,
        move_dir: TileDirection,
        effect_target: EffectTarget,
        rng: &mut impl Rng,
    ) -> Option<Vec<Coords>> {
        let effect_tiles = self.effect_tiles_with_start_tile(target_tile, start, effect_target);
        dijkstra::dijkstra(
            &start,
            |tile| {
                self.neighbours(*tile, Some(target_tile), move_dir, rng)
                    .into_iter()
                    .map(|tile| (tile, 1))
            },
            |tile| effect_tiles.contains(tile),
        )
        .map(|path| path.0.into_iter().skip(1).collect::<Vec<_>>())
        .and_then(|path| (!path.is_empty()).then_some(path))
    }

    #[allow(dead_code)]
    pub fn iter_tiles(&self) -> TileIterator {
        TileIterator::from_size((self.width, self.heigth))
    }

    #[allow(dead_code)]
    pub fn ascii_debug_map(&self) -> String {
        let size = self.grid_size();
        let mut dbg_map = String::with_capacity(size.element_product() as _);
        let x_axis = (0..self.width)
            .map(|i| (i % 10).to_string())
            .collect::<String>();
        dbg_map.push_str(&format!(" _{}_\n", &x_axis));
        dbg_map.push_str(" 0");
        let mut prev_y = 0;
        for tile in self.iter_tiles() {
            if tile.y != prev_y {
                prev_y = tile.y;
                dbg_map.push_str(&format!("{}", tile.y - 1));
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
        dbg_map.push_str(&format!("{}\n _{}_", size.y - 1, &x_axis));
        dbg_map
    }
}

fn track_grid_position(
    mut board_q: Query<(&mut Grid, &GlobalTransform), Changed<GlobalTransform>>,
) {
    for (mut board, t) in &mut board_q {
        board.center_global_position = t.translation().truncate();
    }
}

fn add_new_tile_entities_to_grid(
    entity_q: Query<(Entity, &TileEntityKind, &GlobalTransform), Added<TileEntityKind>>,
    mut grid: Single<&mut Grid>,
) {
    for (e, kind, t) in entity_q {
        let tile = or_return!(grid.world_to_tile(t.translation().truncate()));
        or_return!(grid.place_entity(
            TileEntity {
                entity: e,
                kind: *kind,
            },
            tile,
        ));
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

    #[test_case((2, 2), TileDirection::All, 0 => Some(vec![Coords::new(3, 3)]))]
    #[test_case((2, 2), TileDirection::Diagonal, 0 => Some(vec![Coords::new(3, 3)]))]
    #[test_case((1, 0), TileDirection::All, 0 => Some(vec![Coords::new(2, 0), Coords::new(3, 0), Coords::new(4, 1)]))]
    #[test_case((1, 0), TileDirection::Orthogonal, 0 => Some(vec![Coords::new(2, 0), Coords::new(3, 0), Coords::new(4, 0), Coords::new(4, 1)]))]
    #[test_case((1, 0), TileDirection::Diagonal, 0 => Some(vec![Coords::new(0, 1), Coords::new(1, 2), Coords::new(2, 3), Coords::new(3, 2)]))]
    #[test_case((4, 3), TileDirection::Diagonal, 0 => None)]
    #[traced_test]
    fn path_to_reach_effect_target_effect_range_1(
        start: impl Into<Coords>,
        move_dir: TileDirection,
        seed: u64,
    ) -> Option<Vec<Coords>> {
        let grid = test_grid();
        let mut rng = StdRng::seed_from_u64(seed);

        grid.path_to_reach_effect_target(
            start.into(),
            PLAYER_TILE.into(),
            move_dir,
            EffectTarget {
                reach: EffectReach::Range(1),
                direction: EffectDirection::Area,
            },
            &mut rng,
        )
    }

    #[test_case((2, 2), TileDirection::All, 0 => None)]
    #[test_case((1, 0), TileDirection::All, 0 => Some(vec![Coords::new(2, 0), Coords::new(3, 0), Coords::new(4, 0)]))]
    #[test_case((1, 0), TileDirection::Diagonal, 0 => None)]
    #[traced_test]
    fn path_to_reach_effect_target_effect_exact_range_2(
        start: impl Into<Coords>,
        move_dir: TileDirection,
        seed: u64,
    ) -> Option<Vec<Coords>> {
        let grid = test_grid();
        let mut rng = StdRng::seed_from_u64(seed);

        grid.path_to_reach_effect_target(
            start.into(),
            PLAYER_TILE.into(),
            move_dir,
            EffectTarget {
                reach: EffectReach::Exact(2),
                direction: EffectDirection::Orthogonal,
            },
            &mut rng,
        )
    }

    const PLAYER_TILE: (i16, i16) = (4, 2);

    /// test map:
    ///
    /// \_01234\_
    /// 0.!...0
    /// 1.###.1
    /// 2..!.@2
    /// 3.....3
    /// 4...!.4
    /// \_01234\_
    fn test_grid() -> Grid {
        let mut grid = Grid::new(5, 5);
        for tile in [(1, 1), (2, 1), (3, 1)] {
            _ = grid
                .place_entity(
                    TileEntity {
                        entity: Entity::PLACEHOLDER,
                        kind: TileEntityKind::Wall,
                    },
                    tile.into(),
                )
                .expect("Failed to place an obstacle");
        }
        for tile in [(1, 0), (2, 2), (3, 4)] {
            _ = grid
                .place_entity(
                    TileEntity {
                        entity: Entity::PLACEHOLDER,
                        kind: TileEntityKind::Enemy,
                    },
                    tile.into(),
                )
                .expect("Failed to place an enemy");
        }
        _ = grid
            .place_entity(
                TileEntity {
                    entity: Entity::PLACEHOLDER,
                    kind: TileEntityKind::Player,
                },
                PLAYER_TILE.into(),
            )
            .expect("Failed to place the player");

        println!("{}", grid.ascii_debug_map());
        grid
    }
}
