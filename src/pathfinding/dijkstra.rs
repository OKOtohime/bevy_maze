use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub type DijkstraState = BestFirstState<DijkstraAlgo>;

pub fn setup_dijkstra(
    mut state: ResMut<DijkstraState>, 
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
    state.priority_queue.push(HeapNode { position: start_pos, priority: 0 });
    state.g_score[map.at_pos(&start_pos)] = 0;

    info!("Use Dijkstra Algorithm");
}

pub fn step_dijkstra(
    mut commands: Commands,
    map: Res<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<DijkstraState>,
    mut tracker: ResMut<PathTracker>,
    mut ev_finished: MessageWriter<PathfindingFinished>,
    config: Res<Config>,
) {
    step_best_first_logic(
        &mut commands, &map, &map_view, &mut tracker, &mut ev_finished,
        &mut state,
        |_| 0,
        &config
    );
}