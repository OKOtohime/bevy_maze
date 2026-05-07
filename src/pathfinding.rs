use std::cmp::Ordering;
use crate::core::{is_ready_to_step, AlgorithmSelection, AppState, Map, MapView, Position, SolAlgorithm, TileState, TileType, TileUpdated};
use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::collections::{BinaryHeap, VecDeque};

pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BFSState>()
            .init_resource::<BestFirstState>()
            .init_resource::<PathTracker>()
            .add_systems(OnEnter(AppState::Sol), (
                clear_previous_path,
                setup_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                setup_best_first.run_if(is_best_first),
            ).chain())
            .add_systems(Update, (
                step_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                step_best_first.run_if(is_best_first),
            ).run_if(in_state(AppState::Sol).and(is_searching).and(is_ready_to_step)))
            .add_systems(Update, draw_shortest_path.run_if(in_state(AppState::Sol).and(is_backtracking).and(is_ready_to_step)));
    }
}

fn is_sol_algo(expected: SolAlgorithm) -> impl FnMut(Res<AlgorithmSelection>) -> bool + Clone {
    move |selection: Res<AlgorithmSelection>| selection.sol_algorithm == expected
}

fn is_best_first(selection: Res<AlgorithmSelection>) -> bool {
    matches!(selection.sol_algorithm, SolAlgorithm::AStar | SolAlgorithm::Dijkstra)
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
    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            let tile = map.get_tile(x, y);
            commands.trigger(TileUpdated {
                entity: map_view.get_entity(x, y),
                state: TileState::Terrain(tile)
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
        let entity = map_view.get_entity(current.x, current.y);
        if matches!(map.get_tile_at_pos(&current), TileType::Passable(_)) {
            commands.trigger(TileUpdated{entity, state: TileState::Visited});
        }
        for next_pos in map.get_neighbors(&current, 1) {
            let target_tile = map.get_tile_at_pos(&next_pos);
            if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
                if !tracker.came_from.contains_key(&next_pos) {
                    state.queue.push_back(next_pos);
                    tracker.came_from.insert(next_pos, current);
                }
            }
        }
    }else{
        next_state.set(AppState::Idle);
    }
}

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

#[derive(Resource, Default)]
struct BestFirstState {
    priority_queue: BinaryHeap<Node>,
    g_score: HashMap<Position, i32>
}

fn setup_best_first(
    mut state: ResMut<BestFirstState>,
    map: Res<Map>,
    selection: Res<AlgorithmSelection>
) {
    state.priority_queue.clear();
    state.g_score.clear();
    let start_pos = Position { x: 1, y: 1 };

    let priority = if selection.sol_algorithm == SolAlgorithm::AStar {
        // A*
        let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);
        start_pos.manhattan_distance(&end_pos)
    } else {
        0 // Dijkstra
    };

    state.priority_queue.push(Node { position: start_pos, priority });
    state.g_score.insert(start_pos, 0);
    info!("Use {:?} Algorithm", selection.sol_algorithm);
}

fn step_best_first(
    mut commands: Commands,
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<BestFirstState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
    selection: Res<AlgorithmSelection>
) {
    let is_astar = selection.sol_algorithm == SolAlgorithm::AStar;
    let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);

    let mut valid_node = None;
    while let Some(node) = state.priority_queue.pop() {
        let current_g = *state.g_score.get(&node.position).unwrap_or(&i32::MAX);
        let expected_f = if is_astar {
            current_g.saturating_add(node.position.manhattan_distance(&end_pos))
        } else {
            current_g
        };
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
        let entity = map_view.get_entity(current.x, current.y);
        if matches!(map.get_tile_at_pos(&current), TileType::Passable(_)) {
            commands.trigger(TileUpdated{entity, state: TileState::Visited});
        }
        for next_pos in map.get_neighbors(&current, 1) {
            let target_tile = map.get_tile_at_pos(&next_pos);
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
                    let priority = if is_astar {
                        temp_g_score + next_pos.manhattan_distance(&end_pos)
                    } else {
                        temp_g_score
                    };
                    state.priority_queue.push(Node {
                        position: next_pos,
                        priority
                    });
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
            let entity = map_view.get_entity(parent.x, parent.y);
            commands.trigger(TileUpdated { entity, state: TileState::Path });
            tracker.backtrack = Some(parent);
        }
    }
}
