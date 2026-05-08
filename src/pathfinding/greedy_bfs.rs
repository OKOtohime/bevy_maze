use super::common::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub struct GreedyBFSPlugin;

impl Plugin for GreedyBFSPlugin {
    fn build(&self, app: &mut App) {
        app.register_sol_algo::<GreedyBFSState, _, _>("Greedy BFS", setup_greedy_bfs);
    }
}

pub struct GreedyBFSAlgo;
pub type GreedyBFSState = BestFirstState<GreedyBFSAlgo>;

pub fn setup_greedy_bfs(
    mut state: ResMut<GreedyBFSState>,
    map: Res<Map>,
    config: Res<Config>,
) {
    setup_best_first_logic(&mut state, &map, &config);
    let initial_heuristic = (config.start_pos - config.end_pos).abs().element_sum();
    state.priority_queue.push(HeapNode { position: config.start_pos, priority: initial_heuristic });
    info!("Use Greedy BFS Algorithm");
}

impl SteppedSolAlgorithm for GreedyBFSState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult {
        let end_pos = config.end_pos;
        step_best_first_logic(
            self, map, tracker,
            |pos, _g| (pos - end_pos).abs().element_sum(),
            config
        )
    }
}