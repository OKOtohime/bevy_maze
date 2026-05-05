use std::cmp::Ordering;
use crate::core::{AlgorithmSelection, AppState, Map, MapView, Position, SolAlgorithm, TileType, PaintTile, TIMER_INTERVAL};
use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::collections::{BinaryHeap, VecDeque};
use std::time::Duration;
use bevy::time::common_conditions::on_timer;
use crate::ui::{get_color_for_tile, COLOR_PATH, COLOR_VISITED};

pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BFSState>()
            .init_resource::<AStarState>()
            .init_resource::<DijkstraState>()
            .init_resource::<PathTracker>()
            .add_systems(OnEnter(AppState::Sol), (
                clear_previous_path,
                setup_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                setup_astar.run_if(is_sol_algo(SolAlgorithm::AStar)),
                setup_dijkstra.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
            ).chain())
            .add_systems(Update, (
                step_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                step_astar.run_if(is_sol_algo(SolAlgorithm::AStar)),
                step_dijkstra.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
            ).run_if(in_state(AppState::Sol).and(is_searching).and(on_timer(Duration::from_millis(TIMER_INTERVAL)))))
            .add_systems(Update, draw_shortest_path.run_if(in_state(AppState::Sol).and(is_backtracking).and(on_timer(Duration::from_millis(TIMER_INTERVAL)))));
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
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut tracker: ResMut<PathTracker>,
) {
    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.tiles[y][x];
            commands.trigger(PaintTile {
                entity: map_view.entities[y][x],
                color: get_color_for_tile(tile),
            });
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
    info!("Use BFS Algorithm")
}

fn step_bfs(
    mut commands: Commands,
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<BFSState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
) {
    if let Some(current) = state.queue.pop_front() {
        let end_pos = Position { x: (map.width - 2) as i32, y: (map.height - 2) as i32 };
        if current == end_pos {
            tracker.backtrack = Some(current);
            return;
        }
        let entity = map_view.entities[current.y as usize][current.x as usize];
        if matches!(map.tiles[current.y as usize][current.x as usize], TileType::Passable(_)) {
            commands.trigger(PaintTile{entity, color: COLOR_VISITED});
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                let next_pos = Position{x: nx, y: ny};
                let target_tile = map.tiles[ny as usize][nx as usize];
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
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
#[derive(Debug, PartialEq, Eq)]
struct Node{
    position: Position,
    priority: i32
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Resource)]
struct AStarState{
    priority_queue: BinaryHeap<Node>,
    g_score: HashMap<Position, i32>
}

impl Default for AStarState {
    fn default() -> Self {
        Self{
            priority_queue: BinaryHeap::new(),
            g_score: HashMap::new()
        }
    }
}

fn setup_astar(
    mut state: ResMut<AStarState>,
    map: Res<Map>,
) {
    state.priority_queue.clear();
    state.g_score.clear();
    let start_pos = Position { x: 1, y: 1 };
    let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);
    state.priority_queue.push(Node { position: start_pos, priority: start_pos.manhattan_distance(&end_pos) });
    state.g_score.insert(start_pos, 0);
    info!("Use A* Algorithm")
}

fn step_astar(
    mut commands: Commands,
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<AStarState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
){
    let end_pos = Position{ x: (map.width - 2) as i32, y: (map.height - 2) as i32 };
    let mut valid_node = None;
    while let Some(node) = state.priority_queue.pop() {
        let current_g = *state.g_score.get(&node.position).unwrap_or(&i32::MAX);
        let expected_f = current_g.saturating_add(node.position.manhattan_distance(&end_pos));
        if node.priority <= expected_f {
            valid_node = Some(node);
            break;
        }
    }
    if let Some(node) = valid_node {
        let current = node.position;
        if current == end_pos {
            tracker.backtrack = Some(current);
            return;
        }
        let current_g = *state.g_score.get(&current).unwrap_or(&i32::MAX);
        let entity = map_view.entities[current.y as usize][current.x as usize];
        if matches!(map.tiles[current.y as usize][current.x as usize], TileType::Passable(_)) {
            commands.trigger(PaintTile{entity, color: COLOR_VISITED});
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                let next_pos = Position{x: nx, y: ny};
                let target_tile = map.tiles[ny as usize][nx as usize];
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
                    let step_cost = match target_tile {
                        TileType::Passable(cost) => cost,
                        TileType::End => 1,
                        _ => unreachable!(),
                    };
                    let temp_g_score = current_g + step_cost;
                    let next_g_score = *state.g_score.get(&next_pos).unwrap_or(&i32::MAX);
                    if temp_g_score < next_g_score {
                        tracker.came_from.insert(next_pos, current);
                        state.g_score.insert(next_pos, temp_g_score);
                        let f_score = temp_g_score + next_pos.manhattan_distance(&end_pos);
                        state.priority_queue.push(Node {
                            position: Position { x: nx, y: ny },
                            priority: f_score
                        });
                    }
                }
            }
        }
    }else{
        next_state.set(AppState::Idle);
    }
}

#[derive(Resource)]
struct DijkstraState{
    priority_queue: BinaryHeap<Node>,
    g_score: HashMap<Position, i32>
}

impl Default for DijkstraState {
    fn default() -> Self {
        Self{
            priority_queue: BinaryHeap::new(),
            g_score: HashMap::new()
        }
    }
}

fn setup_dijkstra(mut state: ResMut<DijkstraState>, ) {
    state.priority_queue.clear();
    state.g_score.clear();
    let start_pos = Position { x: 1, y: 1 };
    state.priority_queue.push(Node { position: start_pos, priority: 0 });
    state.g_score.insert(start_pos, 0);
    info!("Use Dijkstra Algorithm")
}

fn step_dijkstra(
    mut commands: Commands,
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<DijkstraState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
){
    let mut valid_node = None;
    while let Some(node) = state.priority_queue.pop() {
        let current_g = *state.g_score.get(&node.position).unwrap_or(&i32::MAX);
        if node.priority <= current_g {
            valid_node = Some(node);
            break;
        }
    }
    if let Some(node) = valid_node {
        let current = node.position;
        let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);
        if current == end_pos {
            tracker.backtrack = Some(current);
            return;
        }
        let current_g = *state.g_score.get(&current).unwrap_or(&i32::MAX);
        let entity = map_view.entities[current.y as usize][current.x as usize];
        if matches!(map.tiles[current.y as usize][current.x as usize], TileType::Passable(_)) {
            commands.trigger(PaintTile{entity, color: COLOR_VISITED});
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                let next_pos = Position::new(nx, ny);
                let target_tile = map.tiles[ny as usize][nx as usize];
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
                    let step_cost = match target_tile {
                        TileType::Passable(cost) => cost,
                        TileType::End => 1,
                        _ => unreachable!(),
                    };
                    let temp_g_score = current_g + step_cost;
                    let next_g_score = *state.g_score.get(&next_pos).unwrap_or(&i32::MAX);
                    if temp_g_score < next_g_score {
                        tracker.came_from.insert(next_pos, current);
                        state.g_score.insert(next_pos, temp_g_score);
                        state.priority_queue.push(Node {
                            position: next_pos,
                            priority: temp_g_score
                        });
                    }
                }
            }
        }
    } else {
        next_state.set(AppState::Idle);
    }
}

// Once the shortest path is found, we would use track to highlight the path
#[derive(Resource, Default)]
pub struct PathTracker {
    pub came_from: HashMap<Position, Position>,
    pub backtrack: Option<Position>,
}

fn draw_shortest_path(
    mut commands: Commands,
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
            commands.trigger(PaintTile { entity, color: COLOR_PATH });
            tracker.backtrack = Some(parent);
        }
    }
}
