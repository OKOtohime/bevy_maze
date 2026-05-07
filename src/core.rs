use std::time::Duration;
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
            .init_resource::<Config>()
            .insert_resource(Map::new(self.width, self.height))
            .add_systems(Update, tick_step_timer);
    }
}

impl Default for CorePlugin {
    fn default() -> Self {
        Self{ width: 20, height: 20 }
    }
}

// To visualize the algorithm process, we have to run the algorithm step by step
#[derive(Resource)]
pub struct Config {
    pub step_timer: Timer,
    pub mud_chance: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            step_timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
            mud_chance: 0.05,
        }
    }
}

fn tick_step_timer(time: Res<Time>, mut config: ResMut<Config>) {
    config.step_timer.tick(time.delta());
}

pub fn is_ready_to_step(config: Res<Config>) -> bool {
    config.step_timer.just_finished()
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
    pub tiles: Vec<TileType> // real size: 2*width+1, 2*height+1
}

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

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let real_width = (width<<1) + 1;
        let real_height = (height<<1) + 1;
        let tiles = vec![TileType::Barrier; real_width*real_height];
        Self {
            width: real_width,
            height: real_height,
            tiles
        }
    }

    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x > 0 && x < (self.width - 1) as i32 && y > 0 && y < (self.height - 1) as i32
    }

    pub fn get_neighbors(&self, pos: &Position, step: i32) -> Vec<Position> {
        let direction = [(0, step), (step, 0), (0, -step), (-step, 0)];
        direction.iter().map(|(dx, dy)| Position{x:pos.x + dx, y:pos.y + dy})
            .filter(|p| self.is_inside(p.x, p.y)).collect()
    }
    pub fn at(&self, x: i32, y: i32) -> usize {
        (y as usize) * self.width + (x as usize)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        self.tiles[self.at(x, y)]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        let idx = self.at(x, y);
        self.tiles[idx] = tile;
    }

    pub fn get_tile_at_pos(&self, pos: &Position) -> TileType {
        self.tiles[self.at(pos.x, pos.y)]
    }

    pub fn set_tile_at_pos(&mut self, pos: &Position, tile: TileType) {
        let idx = self.at(pos.x, pos.y);
        self.tiles[idx] = tile;
    }
}

// map: (x, y) -> entity
#[derive(Resource)]
pub struct MapView {
    pub width: usize,
    pub height: usize,
    pub entities: Vec<Entity>,
}

impl MapView {
    pub fn at(&self, x: i32, y: i32) -> usize {
        (y as usize) * self.width + (x as usize)
    }

    pub fn at_pos(&self, pos: Position) -> usize {
        self.at(pos.x, pos.y)
    }

    pub fn get_entity(&self, x: i32, y: i32) -> Entity {
        self.entities[self.at(x, y)]
    }

    pub fn set_entity(&mut self, x: i32, y: i32, entity: Entity) {
        let idx = self.at(x, y);
        self.entities[idx] = entity;
    }
}
