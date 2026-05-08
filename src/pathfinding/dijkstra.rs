use super::common::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub struct DijkstraPlugin;

impl Plugin for DijkstraPlugin{
    fn build(&self, app: &mut App){
        app.register_sol_algo::<DijkstraState, _, _>("Dijkstra", setup_dijkstra);
    }
}

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
            |_pos, g| g,
            &config
        )
    }
}
