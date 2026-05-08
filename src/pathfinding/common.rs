use crate::core::prelude::*;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub struct HeapNode {
    pub position: IVec2,
    pub priority: i32
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Once the shortest path is found, we would use track to highlight the path
#[derive(Resource, Default)]
pub struct PathTracker {
    pub came_from: Vec<Option<IVec2>>,
    pub backtrack: Option<IVec2>,
}

// Shared logic for dijkstra and astar
pub struct AStarAlgo;
pub struct DijkstraAlgo;

#[derive(Resource)]
pub struct BestFirstState<T: Send + Sync + 'static> {
    pub priority_queue: BinaryHeap<HeapNode>,
    pub g_score: Vec<i32>,
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Default for BestFirstState<T> {
    fn default() -> Self {
        Self {
            priority_queue: BinaryHeap::new(),
            g_score: Vec::new(),
            _marker: PhantomData,
        }
    }
}

pub fn step_best_first_logic<T: Send + Sync + 'static>(
    commands: &mut Commands,
    map: &Map,
    map_view: &MapView,
    tracker: &mut PathTracker,
    next_state: &mut NextState<AppState>,
    state: &mut BestFirstState<T>,
    heuristic: impl Fn(IVec2) -> i32,
    config: &Config,
) {
    let end_pos = config.end_pos;
    let mut valid_node = None;

    while let Some(node) = state.priority_queue.pop() {
        let current_g = state.g_score[map.at_pos(&node.position)];
        let expected_f = current_g.saturating_add(heuristic(node.position));
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
        let current_g = state.g_score[map.at_pos(&current)];
        let tile = *map.get(current.x, current.y);
        if matches!(tile, TileType::Passable(_)) {
            commands.trigger(TileUpdated {
                entity: *map_view.get(current.x, current.y),
                state: TileState::Visited
            });
        }
        for next_pos in map.get_neighbors(&current, 1) {
            let target_tile = *map.get(next_pos.x, next_pos.y);
            if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
                let step_cost = match target_tile {
                    TileType::Passable(cost) => cost,
                    TileType::End => 1,
                    _ => unreachable!(),
                };
                let temp_g_score = current_g + step_cost;
                let next_idx = map.at_pos(&next_pos);
                if temp_g_score < state.g_score[next_idx] {
                    tracker.came_from[next_idx] = Some(current);
                    state.g_score[next_idx] = temp_g_score;

                    state.priority_queue.push(HeapNode {
                        position: next_pos,
                        priority: temp_g_score + heuristic(next_pos),
                    });
                }
            }
        }
    } else {
        next_state.set(AppState::Idle);
    }
}

pub fn clear_previous_path(
    mut commands: Commands,
    map: ResMut<Map>,
    map_view: Res<MapView>,
    mut tracker: ResMut<PathTracker>,
    config: Res<Config>,
) {
    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            let tile = *map.get(x, y);
            commands.trigger(TileUpdated {
                entity: *map_view.get(x, y),
                state: TileState::Terrain(tile)
            });
        }
    }
    let size = map.width * map.height;
    if tracker.came_from.len() != size {
        tracker.came_from = vec![None; size];
    } else {
        tracker.came_from.fill(None);
    }
    tracker.backtrack = None;
    let start_pos = config.start_pos;
    tracker.came_from[map.at_pos(&start_pos)] = Some(start_pos);
}

pub fn draw_shortest_path(
    mut commands: Commands,
    map: Res<Map>,
    map_view: Res<MapView>,
    mut tracker: ResMut<PathTracker>,
    mut next_app_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    if let Some(current_backtrack) = tracker.backtrack {
        if let Some(parent) = tracker.came_from[map.at_pos(&current_backtrack)] {
            if parent == (config.start_pos) {
                tracker.backtrack = None;
                next_app_state.set(AppState::Idle);
                return;
            }
            commands.trigger(TileUpdated {
                entity: *map_view.get_at_pos(&parent),
                state: TileState::Path
            });
            tracker.backtrack = Some(parent);
        }
    }
}
