use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct BFSState {
    pub queue: VecDeque<IVec2>,
}

pub fn setup_bfs(mut state: ResMut<BFSState>, config: Res<Config>) {
    state.queue.clear();
    state.queue.push_back(config.start_pos);
    info!("Use BFS Algorithm");
}

pub fn step_bfs(
    mut commands: Commands,
    map: Res<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<BFSState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut tracker: ResMut<PathTracker>,
    config: Res<Config>,
) {
    if let Some(current) = state.queue.pop_front() {
        let end_pos = config.end_pos;
        if current == end_pos {
            tracker.backtrack = Some(current);
            return;
        }
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
                let next_idx = map.at_pos(&next_pos);
                if tracker.came_from[next_idx].is_none() {
                    state.queue.push_back(next_pos);
                    tracker.came_from[next_idx] = Some(current);
                }
            }
        }
    } else {
        next_state.set(AppState::Idle);
    }
}