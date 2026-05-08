use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;
use std::collections::BinaryHeap;

pub type AStarState = BestFirstState<AStarAlgo>;

pub fn setup_astar(
    mut state: ResMut<AStarState>, 
    map: Res<Map>,
    config: Res<Config>,
) {
    state.priority_queue.clear();
    let size = map.width * map.height;
    if state.g_score.len() != size {
        state.g_score = vec![i32::MAX; size];
    } else {
        state.g_score.fill(i32::MAX);
    }

    let start_pos = config.start_pos;
    let end_pos = config.end_pos;
    let initial_heuristic = (start_pos - end_pos).abs().element_sum();

    state.priority_queue.push(HeapNode { position: start_pos, priority: initial_heuristic });
    state.g_score[map.at_pos(&start_pos)] = 0;

    info!("Use A* Algorithm");
}

pub fn step_astar(
    mut commands: Commands,
    map: Res<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<AStarState>,
    mut tracker: ResMut<PathTracker>,
    mut next_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    let end_pos = IVec2::new((map.width - 2) as i32, (map.height - 2) as i32);
    step_best_first_logic(
        &mut commands, &map, &map_view, &mut tracker, &mut next_state,
        &mut state,
        |pos| (pos - end_pos).abs().element_sum(),
        &config
    );
}