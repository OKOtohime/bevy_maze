use super::common::*;
use crate::core::prelude::*;
use bevy::prelude::*;

pub struct WallFollowerPlugin;

impl Plugin for WallFollowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_sol_algo::<WallFollowerState, _, _>("Wall Follower", setup_wall_follower);
    }
}

const DIRS: [IVec2; 4] = [
    IVec2::new(0, -1), // 0: Up
    IVec2::new(1, 0),  // 1: Right
    IVec2::new(0, 1),  // 2: Down
    IVec2::new(-1, 0), // 3: Left
];

#[derive(Resource, Default)]
pub struct WallFollowerState {
    pub current: IVec2,
    pub dir_idx: usize, // direction facing currently
    pub finished: bool,
}

pub fn setup_wall_follower(mut state: ResMut<WallFollowerState>, config: Res<Config>) {
    state.current = config.start_pos;
    state.dir_idx = 1;
    state.finished = false;
    info!("Use Wall Follower Algorithm");
}

impl SteppedSolAlgorithm for WallFollowerState {
    fn step(&mut self, map: &Map, config: &Config, tracker: &mut PathTracker) -> SolStepResult {
        if self.finished { return SolStepResult::Finished; }

        let current = self.current;
        if current == config.end_pos {
            tracker.backtrack = Some(current);
            self.finished = true;
            return SolStepResult::Found(current);
        }

        let turn_order = [
            (self.dir_idx + 1) % 4, // turn right
            self.dir_idx,           // go straight
            (self.dir_idx + 3) % 4, // turn left
            (self.dir_idx + 2) % 4, // turn around
        ];

        for &next_dir in &turn_order {
            let next_pos = current + DIRS[next_dir];
            if map.is_inside(next_pos.x, next_pos.y) {
                let target_tile = *map.get(next_pos.x, next_pos.y);
                if matches!(target_tile, TileType::Passable(_)) || target_tile == TileType::End || target_tile == TileType::Start {
                    self.current = next_pos;
                    self.dir_idx = next_dir;
                    let next_idx = map.at_pos(&next_pos);
                    if tracker.came_from[next_idx].is_none() {
                        tracker.came_from[next_idx] = Some(current);
                    }
                    return SolStepResult::Visited(next_pos);
                }
            }
        }
        self.finished = true;
        SolStepResult::Finished
    }
}