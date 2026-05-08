pub mod common;
pub mod bfs;
pub mod dijkstra;
pub mod astar;
pub mod bibfs;
mod prelude;
mod greedy_bfs;
mod wall_follower;

use crate::core::prelude::*;
use bevy::prelude::*;
use prelude::*;


pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathTracker>()
            .add_systems(OnEnter(AppState::Sol), clear_previous_path)
            .add_plugins((
                BFSPlugin,
                BiBFSPlugin,
                DijkstraPlugin,
                AStarPlugin,
                GreedyBFSPlugin,
                WallFollowerPlugin,
            ));
    }
}
