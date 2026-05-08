use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub type DijkstraState = BestFirstState<DijkstraAlgo>;

pub fn setup_dijkstra(
    mut state: ResMut<DijkstraState>, 
    map: Res<Map>,
    config: Res<Config>,
) {
    setup_best_first_logic(&mut state, &map, &config);
    state.priority_queue.push(HeapNode { position: config.start_pos, priority: 0 });
    info!("Use Dijkstra Algorithm");
}

impl SteppedSolAlgorithm for DijkstraState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult {
        step_best_first_logic(
            self,
            &map,
            tracker,
            |_| 0,
            &config
        )
    }
}
