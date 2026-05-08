use bevy::prelude::*;
use rand::prelude::IndexedRandom;
use crate::core::prelude::*;
use super::prelude::*;

#[derive(Resource, Default)]
pub struct DFSGenState {
    pub stack: Vec<Position>,
}

pub fn setup_dfs(mut state: ResMut<DFSGenState>) {
    state.stack.clear();
    state.stack.push(Position{ x: 1, y: 1 });
    info!("Use DFS Algorithm");
}

pub fn step_dfs(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<DFSGenState>,
    mut next_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    let mut rng = rand::rng();
    if let Some(current) = state.stack.last().copied() {
        let unvisited_neighbors: Vec<Position> = map
            .get_neighbors(&current, 2)
            .filter(|pos| *map.get_at_pos(pos) == TileType::Barrier)
            .collect();
        if !unvisited_neighbors.is_empty() {
            let &next_pos = unvisited_neighbors.choose(&mut rng).unwrap();
            let wall_pos = Position::new((current.x + next_pos.x) >> 1, (current.y + next_pos.y) >> 1);
            update_map_at_pos(&mut commands, &mut map, &map_view, wall_pos, TileType::Passable(1));
            update_map_at_pos(&mut commands, &mut map, &map_view, next_pos, TileType::Passable(1));
            state.stack.push(next_pos);
        } else {
            state.stack.pop();
        }
    }else{
        finish_generation(&mut commands, &mut map, &map_view, &mut next_state, &config);
    }
}