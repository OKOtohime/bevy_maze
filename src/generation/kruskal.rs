use bevy::prelude::*;
use crate::core::prelude::*;
use super::prelude::*;
use rand::prelude::SliceRandom;

#[derive(Resource, Default)]
pub struct KruskalGenState {
    pub walls: Vec<IVec2>,
    pub parent: Vec<usize>,
}

fn find(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

pub fn setup_kruskal(mut state: ResMut<KruskalGenState>, map: Res<Map>) {
    state.walls.clear();
    let total_cells = map.width * map.height;
    state.parent = (0..total_cells).collect();

    for y in (1..map.height as i32 - 1).step_by(2) {
        for x in (1..map.width as i32 - 1).step_by(2) {
            if x + 2 < map.width as i32 {
                state.walls.push(IVec2::new(x + 1, y));
            }
            if y + 2 < map.height as i32 {
                state.walls.push(IVec2::new(x, y + 1));
            }
        }
    }
    let mut rng = rand::rng();
    state.walls.shuffle(&mut rng);
    info!("Use Kruskal's Algorithm");
}

impl SteppedGenAlgorithm for KruskalGenState {
    fn step(&mut self, map: &Map, _config: &Config) -> GenStepResult {
        while let Some(wall) = self.walls.pop() {
            let (cell1, cell2) = if wall.x % 2 == 0 {
                (IVec2::new(wall.x - 1, wall.y), IVec2::new(wall.x + 1, wall.y))
            } else {
                (IVec2::new(wall.x, wall.y - 1), IVec2::new(wall.x, wall.y + 1))
            };
            let root1 = find(&mut self.parent, map.at_pos(&cell1));
            let root2 = find(&mut self.parent, map.at_pos(&cell2));
            if root1 != root2 {
                self.parent[root1] = root2;
                return GenStepResult::TilesModified(vec![wall, cell1, cell2])
            }
        }
        GenStepResult::Finished
    }
}
