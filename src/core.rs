use bevy::app::App;
use bevy::prelude::*;

pub struct CorePlugin {
    pub width: usize,
    pub height: usize,
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<AlgorithmSelection>()
            .insert_resource(Map::new(self.width, self.height));
    }
}

impl Default for CorePlugin {
    fn default() -> Self {
        Self{ width: 20, height: 20 }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Gen,
    Sol,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenAlgorithm {
    #[default]
    DFS,
    Prim,
    Kruskal,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SolAlgorithm {
    #[default]
    BFS,
    Dijkstra,
    AStar,
}

#[derive(Resource, Default, PartialEq, Eq, Debug)]
pub struct AlgorithmSelection {
    pub gen_algorithm: GenAlgorithm,
    pub sol_algorithm: SolAlgorithm,
}

// Coordinate of node in world
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Ord, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn manhattan_distance(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

// 4-direction square map
#[derive(Resource)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<TileType>> // real size: 2*width+1, 2*height+1
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileType {
    Barrier,
    Passable,
    Start,
    End,
    Visited,
    ShortestPath
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let real_width = (width<<1) + 1;
        let real_height = (height<<1) + 1;
        let tiles = vec![vec![TileType::Barrier; real_width]; real_height];
        Self {
            width: real_width,
            height: real_height,
            tiles
        }
    }
}

// map: (x, y) -> entity
#[derive(Resource)]
pub struct MapView {
    pub entities: Vec<Vec<Entity>>,
}

// would trigger on specific entity
// only observer that listens this entity would execute
#[derive(EntityEvent)]
pub struct UpdateTile {
    pub entity: Entity,
    pub new_type: TileType,
}

// To visualize the algorithm process, we have to run the algorithm step by step
pub const TIMER_INTERVAL: u64 = 10;
