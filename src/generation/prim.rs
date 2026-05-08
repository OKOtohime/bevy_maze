use bevy::prelude::*;
use crate::core::prelude::*;
use super::prelude::*;

#[derive(Resource, Default)]
pub struct PrimGenState {
    pub frontier: Vec<(Position, Position)>, // (Wall, NextCell)
}

pub fn setup_prim(mut state: ResMut<PrimGenState>, map: Res<Map>) {
    state.frontier.clear();
    let start = Position::new(1, 1);
    for next_pos in map.get_neighbors(&start, 2) {
        state.frontier.push((Position::new((start.x + next_pos.x)>>1, (start.y + next_pos.y)>>1), next_pos));
    }
    info!("Use Prim's Algorithm");
}

pub fn step_prim(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<PrimGenState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    while !state.frontier.is_empty() {
        let idx = rand::random_range(0..state.frontier.len());
        let (wall, next_cell) = state.frontier.swap_remove(idx);
        if *map.get_at_pos(&next_cell) == TileType::Barrier {
            update_map_at_pos(&mut commands, &mut map, &map_view, wall, TileType::Passable(1));
            update_map_at_pos(&mut commands, &mut map, &map_view, next_cell, TileType::Passable(1));
            for next_pos in map.get_neighbors(&next_cell, 2) {
                if *map.get_at_pos(&next_pos) == TileType::Barrier {
                    state.frontier.push((Position::new((next_cell.x + next_pos.x)>>1, (next_cell.y + next_pos.y)>>1), next_pos));
                }
            }
            return;
        }
    }
    finish_generation(&mut commands, &mut map, &map_view, &mut next_app_state, &config);
}
