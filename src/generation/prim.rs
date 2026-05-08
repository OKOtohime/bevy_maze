use bevy::prelude::*;
use crate::core::prelude::*;
use super::prelude::*;

#[derive(Resource, Default)]
pub struct PrimGenState {
    pub frontier: Vec<(IVec2, IVec2)>, // (Wall, NextCell)
}

pub fn setup_prim(
    mut state: ResMut<PrimGenState>, 
    map: Res<Map>,
    config: Res<Config>,
) {
    state.frontier.clear();
    let start = config.start_pos;
    for next_pos in map.get_neighbors(&start, 2) {
        state.frontier.push((IVec2::new((start.x + next_pos.x)>>1, (start.y + next_pos.y)>>1), next_pos));
    }
    info!("Use Prim's Algorithm");
}

impl SteppedGenAlgorithm for PrimGenState {
    fn step(&mut self, map: &Map, _config: &Config) -> GenStepResult {
        while !self.frontier.is_empty() {
            let idx = rand::random_range(0..self.frontier.len());
            let (wall, next_cell) = self.frontier.swap_remove(idx);
            if *map.get_at_pos(&next_cell) == TileType::Barrier {
                for next_pos in map.get_neighbors(&next_cell, 2) {
                    if *map.get_at_pos(&next_pos) == TileType::Barrier {
                        self.frontier.push((IVec2::new((next_cell.x + next_pos.x)>>1, (next_cell.y + next_pos.y)>>1), next_pos));
                    }
                }
                return GenStepResult::TilesModified(vec![wall, next_cell])
            }
        }
        GenStepResult::Finished
    }
}
