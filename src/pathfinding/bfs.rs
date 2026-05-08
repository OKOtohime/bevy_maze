use super::common::*;
use crate::core::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct BFSPlugin;

impl Plugin for BFSPlugin{
    fn build(&self, app: &mut App){
        app.register_sol_algo::<BFSState, _, _>("BFS", setup_bfs);
    }
}

#[derive(Resource, Default)]
pub struct BFSState {
    pub queue: VecDeque<IVec2>,
}

pub fn setup_bfs(mut state: ResMut<BFSState>, config: Res<Config>) {
    state.queue.clear();
    state.queue.push_back(config.start_pos);
    info!("Use BFS Algorithm");
}

impl SteppedSolAlgorithm for BFSState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult{
        if let Some(current) = self.queue.pop_front() {
            let end_pos = config.end_pos;
            if current == end_pos {
                tracker.backtrack = Some(current);
                return SolStepResult::Found(end_pos)
            }
            for next_pos in map.get_neighbors(&current, 1) {
                let target_tile = *map.get(next_pos.x, next_pos.y);
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End {
                    let next_idx = map.at_pos(&next_pos);
                    if tracker.came_from[next_idx].is_none() {
                        self.queue.push_back(next_pos);
                        tracker.came_from[next_idx] = Some(current);
                    }
                }
            }
            SolStepResult::Visited(current)
        } else {
            SolStepResult::Finished
        }
    }
}
