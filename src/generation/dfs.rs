use super::prelude::*;
use crate::core::prelude::*;
use bevy::prelude::*;
use rand::prelude::IndexedRandom;

pub struct DFSPlugin;

impl Plugin for DFSPlugin{
    fn build(&self, app: &mut App){
        app.register_gen_algo::<DFSGenState, _, _>("DFS", setup_dfs);
    }
}

#[derive(Resource, Default)]
pub struct DFSGenState {
    pub stack: Vec<IVec2>,
}

pub fn setup_dfs(mut state: ResMut<DFSGenState>, config: Res<Config>) {
    state.stack.clear();
    state.stack.push(config.start_pos);
    info!("Use DFS Algorithm");
}

impl SteppedGenAlgorithm for DFSGenState {
    fn step(&mut self, map: &Map, _config: &Config) -> GenStepResult {
        let mut rng = rand::rng();
        if let Some(current) = self.stack.last().copied() {
            let unvisited_neighbors: Vec<IVec2> = map
                .get_neighbors(&current, 2)
                .filter(|pos| *map.get_at_pos(pos) == TileType::Barrier)
                .collect();
            if !unvisited_neighbors.is_empty() {
                let &next_pos = unvisited_neighbors.choose(&mut rng).unwrap();
                let wall_pos = (current + next_pos) >> 1;
                self.stack.push(next_pos);
                GenStepResult::TilesModified(vec![wall_pos, next_pos])
            } else {
                self.stack.pop();
                GenStepResult::InProgress
            }
        }else{
            GenStepResult::Finished
        }
    }
}
