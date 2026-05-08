pub mod common;
pub mod bfs;
pub mod dijkstra;
pub mod astar;
pub mod prelude;

use bevy::prelude::*;
use crate::core::prelude::*;
use prelude::*;

pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BFSState>()
            .init_resource::<DijkstraState>()
            .init_resource::<AStarState>()
            .init_resource::<PathTracker>()
            .add_systems(OnEnter(AppState::Sol), clear_previous_path)
            .add_systems(OnEnter(ActiveSolState(SolAlgorithm::BFS)), setup_bfs)
            .add_systems(OnEnter(ActiveSolState(SolAlgorithm::Dijkstra)), setup_dijkstra)
            .add_systems(OnEnter(ActiveSolState(SolAlgorithm::AStar)), setup_astar)
            .add_systems(Update, (
                step_sol_algorithm::<BFSState>.run_if(in_state(ActiveSolState(SolAlgorithm::BFS))),
                step_sol_algorithm::<DijkstraState>.run_if(in_state(ActiveSolState(SolAlgorithm::Dijkstra))),
                step_sol_algorithm::<AStarState>.run_if(in_state(ActiveSolState(SolAlgorithm::AStar))),
            ).run_if(is_ready_to_step));
    }
}
