use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub type AStarState = BestFirstState<AStarAlgo>;

pub fn setup_astar(
    mut state: ResMut<AStarState>,
    map: Res<Map>,
    config: Res<Config>,
) {
    setup_best_first_logic(&mut state, &map, &config);
    let initial_heuristic = (config.start_pos - config.end_pos).abs().element_sum();
    state.priority_queue.push(HeapNode { position: config.start_pos, priority: initial_heuristic });
    info!("Use A* Algorithm");
}

impl SteppedSolAlgorithm for AStarState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult {
        let end_pos = config.end_pos;
        step_best_first_logic(
            self,
            &map,
            tracker,
            |pos| (pos - end_pos).abs().element_sum(),
            &config
        )
    }
}
