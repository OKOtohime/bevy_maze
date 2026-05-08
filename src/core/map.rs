use bevy::prelude::*;
use super::prelude::{Grid2D, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Barrier,
    Passable(i32), // cost
    Start,
    End
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileState {
    Terrain(TileType),
    Visited,
    Path,
}

// would trigger on specific entity
// only observer that listens this entity would execute
#[derive(EntityEvent)]
pub struct TileUpdated {
    pub entity: Entity,
    pub state: TileState,
}

pub type Map = Grid2D<TileType>; // Position -> TileType
pub type MapView = Grid2D<Entity>; // Position -> Entity

impl Map {
    pub fn new_maze(width: usize, height: usize) -> Self {
        let real_width = (width<<1) + 1;
        let real_height = (height<<1) + 1;
        Self::new(real_width, real_height, TileType::Barrier)
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x > 0 && x < (self.width - 1) as i32 && y > 0 && y < (self.height - 1) as i32
    }

    pub fn get_neighbors<'a>(&'a self, pos: &Position, step: i32) -> impl Iterator<Item = Position> + 'a {
        let (px, py) = (pos.x, pos.y);
        let direction = [(0, step), (step, 0), (0, -step), (-step, 0)];
        direction.into_iter()
            .map(move |(dx, dy)| Position::new(px + dx, py + dy))
            .filter(move |p| self.is_inside(p.x, p.y))
    }
}
