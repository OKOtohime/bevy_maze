use bevy::prelude::*;
use std::collections::BinaryHeap;
use crate::core::prelude::*;
use super::prelude::*;

pub type AStarState = BestFirstState<AStarAlgo>;

pub fn setup_astar(mut state: ResMut<AStarState>, map: Res<Map>) {
    state.priority_queue.clear();
    let size = map.width * map.height;
    if state.g_score.len() != size {
        state.g_score = vec![i32::MAX; size];
    } else {
        state.g_score.fill(i32::MAX);
    }

    let start_pos = Position::new(1, 1);
    let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);
    let initial_heuristic = start_pos.manhattan_distance(&end_pos);

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
) {
    let end_pos = Position::new((map.width - 2) as i32, (map.height - 2) as i32);
    step_best_first_logic(
        &mut commands, &map, &map_view, &mut tracker, &mut next_state,
        &mut state,
        |pos| pos.manhattan_distance(&end_pos),
    );
}