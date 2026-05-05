use crate::core::{AlgorithmSelection, AppState, Map, MapView, Position, SolAlgorithm, TileType, UpdateTile, TIMER_INTERVAL};
use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::collections::{BinaryHeap, VecDeque};
use std::time::Duration;
use bevy::time::common_conditions::on_timer;

pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BFSState>()
            // .init_resource::<AStarState>()
            // .init_resource::<DijkstraState>()
            .init_resource::<PathTracker>()
            .add_systems(OnEnter(AppState::Sol), (
                clear_previous_path,
                setup_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                // setup_astar.run_if(is_sol_algo(SolAlgorithm::AStar)),
                // setup_dijkstra.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
            ).chain())
            .add_systems(Update, (
                step_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                // step_astar.run_if(is_sol_algo(SolAlgorithm::AStar)),
                // step_dijkstra.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
            ).run_if(in_state(AppState::Sol).and(is_searching).and(on_timer(Duration::from_millis(TIMER_INTERVAL)))))
            .add_systems(Update, draw_shortest_path.run_if(in_state(AppState::Sol).and(is_backtracking)));
    }
}

fn is_sol_algo(expected: SolAlgorithm) -> impl FnMut(Res<AlgorithmSelection>) -> bool + Clone {
    move |selection: Res<AlgorithmSelection>| selection.sol_algorithm == expected
}

fn is_searching(tracker: Res<PathTracker>) -> bool {
    tracker.backtrack.is_none()
}

fn is_backtracking(tracker: Res<PathTracker>) -> bool {
    tracker.backtrack.is_some()
}

fn clear_previous_path(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut tracker: ResMut<PathTracker>,
) {
    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.tiles[y][x];
            if tile == TileType::Visited || tile == TileType::ShortestPath {
                map.tiles[y][x] = TileType::Passable;
                commands.trigger(UpdateTile {
                    entity: map_view.entities[y][x],
                    new_type: TileType::Passable
                });
            }
        }
    }
    tracker.came_from.clear();
    tracker.backtrack = None;
    let start_pos = Position { x: 1, y: 1 };
    tracker.came_from.insert(start_pos, start_pos);
}

// BFS impl
#[derive(Resource)]
pub struct BFSState {
    pub queue: VecDeque<Position>
}

impl Default for BFSState {
    fn default() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }
}

fn setup_bfs(mut state: ResMut<BFSState>) {
    state.queue.clear();
    let start_pos = Position { x: 1, y: 1 };
    state.queue.push_back(start_pos);
}

fn step_bfs(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<BFSState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
) {
    // search path
    if let Some(current) = state.queue.pop_front() {
        if current.y == (map.height - 2) as i32 && current.x == (map.width - 2) as i32 {
            tracker.backtrack = Some(current);
            return;
        }
        let entity = map_view.entities[current.y as usize][current.x as usize];
        if map.tiles[current.y as usize][current.x as usize] == TileType::Passable {
            map.tiles[current.y as usize][current.x as usize] = TileType::Visited;
            commands.trigger(UpdateTile{entity, new_type: TileType::Visited});
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                let next_pos = Position{x: nx, y: ny};
                let target_tile = map.tiles[ny as usize][nx as usize];
                if target_tile == TileType::Passable || target_tile == TileType::End {
                    if !tracker.came_from.contains_key(&next_pos) {
                        state.queue.push_back(Position { x: nx, y: ny });
                        tracker.came_from.insert(next_pos, current);
                    }
                }
            }
        }
    }else{
        next_state.set(AppState::Idle);
    }
}

// A* impl
#[derive(Resource)]
struct AStarState{
    priority_queue: BinaryHeap<(i32, u32)>,

}

// Once the shortest path is found, we would use track to highlight the path
#[derive(Resource, Default)]
pub struct PathTracker {
    pub came_from: HashMap<Position, Position>,
    pub backtrack: Option<Position>,
}

fn draw_shortest_path(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut tracker: ResMut<PathTracker>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Some(current_backtrack) = tracker.backtrack {
        if let Some(&parent) = tracker.came_from.get(&current_backtrack) {
            if parent == (Position { x: 1, y: 1 }) {
                tracker.backtrack = None;
                next_app_state.set(AppState::Idle);
                return;
            }
            let entity = map_view.entities[parent.y as usize][parent.x as usize];
            map.tiles[parent.y as usize][parent.x as usize] = TileType::ShortestPath;
            commands.trigger(UpdateTile { entity, new_type: TileType::ShortestPath });
            tracker.backtrack = Some(parent);
        }
    }
}

