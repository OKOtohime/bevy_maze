use super::common::*;
use crate::core::prelude::*;
use bevy::prelude::*;
use std::collections::VecDeque;

pub struct BiBFSPlugin;

impl Plugin for BiBFSPlugin{
    fn build(&self, app: &mut App){
        app.register_sol_algo::<BiBFSState, _, _>("BiBFS", setup_bibfs);
    }
}

#[derive(Resource, Default)]
pub struct BiBFSState {
    pub fwd_queue: VecDeque<IVec2>,
    pub bwd_queue: VecDeque<IVec2>,
    pub fwd_came_from: Vec<Option<IVec2>>,
    pub bwd_came_from: Vec<Option<IVec2>>,
    pub forward_turn: bool, // step turn by turn
                            // true => forward
                            // false => backward
}

impl SteppedSolAlgorithm for BiBFSState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult {
        if self.fwd_queue.is_empty() && !self.bwd_queue.is_empty() {
            self.forward_turn = false;
        } else if !self.fwd_queue.is_empty() && self.bwd_queue.is_empty() {
            self.forward_turn = true;
        } else if self.fwd_queue.is_empty() && self.bwd_queue.is_empty() {
            return SolStepResult::Finished;
        }
        self.forward_turn = !self.forward_turn;
        let (current_queue, current_came_from, other_came_from) = if self.forward_turn {
            (&mut self.fwd_queue, &mut self.fwd_came_from, &self.bwd_came_from)
        } else {
            (&mut self.bwd_queue, &mut self.bwd_came_from, &self.fwd_came_from)
        };
        if let Some(current) = current_queue.pop_front() {
            for next_pos in map.get_neighbors(&current, 1) {
                let target_tile = *map.get(next_pos.x, next_pos.y);
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End || target_tile == TileType::Start {
                    let next_idx = map.at_pos(&next_pos);
                    if other_came_from[next_idx].is_some() {
                        current_came_from[next_idx] = Some(current);
                        concat_paths(map, config, tracker, &self.fwd_came_from, &self.bwd_came_from, next_pos);
                        return SolStepResult::Found(config.end_pos);
                    }
                    if current_came_from[next_idx].is_none() {
                        current_queue.push_back(next_pos);
                        current_came_from[next_idx] = Some(current);
                    }
                }
            }
            return SolStepResult::Visited(current);
        }
        SolStepResult::InProgress
    }
}

pub fn setup_bibfs(mut state: ResMut<BiBFSState>, map: Res<Map>, config: Res<Config>) {
    state.fwd_queue.clear();
    state.bwd_queue.clear();

    let size = map.width * map.height;
    state.fwd_came_from.clear();
    state.fwd_came_from.resize(size, None);
    state.bwd_came_from.clear();
    state.bwd_came_from.resize(size, None);

    state.fwd_queue.push_back(config.start_pos);
    state.fwd_came_from[map.at_pos(&config.start_pos)] = Some(config.start_pos);

    state.bwd_queue.push_back(config.end_pos);
    state.bwd_came_from[map.at_pos(&config.end_pos)] = Some(config.end_pos);

    state.forward_turn = true;
    info!("Use Bidirectional BFS Algorithm");
}

fn concat_paths(
    map: &Map,
    config: &Config,
    tracker: &mut PathTracker,
    fwd_came_from: &[Option<IVec2>],
    bwd_came_from: &[Option<IVec2>],
    meet_point: IVec2,
) {
    for (i, parent) in fwd_came_from.iter().enumerate() {
        if let Some(p) = parent {
            tracker.came_from[i] = Some(*p);
        }
    }
    let mut curr = meet_point;
    while curr != config.end_pos {
        let next_towards_end = bwd_came_from[map.at_pos(&curr)].unwrap();
        tracker.came_from[map.at_pos(&next_towards_end)] = Some(curr);
        curr = next_towards_end;
    }
}
