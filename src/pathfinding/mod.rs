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
            .add_systems(OnEnter(AppState::Sol), (
                clear_previous_path,
                setup_bfs.run_if(is_sol_algo(SolAlgorithm::BFS)),
                setup_dijkstra.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
                setup_astar.run_if(is_sol_algo(SolAlgorithm::AStar)),
            ).chain())
            .add_systems(Update, (
                step_sol_algorithm::<BFSState>.run_if(is_sol_algo(SolAlgorithm::BFS)),
                step_sol_algorithm::<DijkstraState>.run_if(is_sol_algo(SolAlgorithm::Dijkstra)),
                step_sol_algorithm::<AStarState>.run_if(is_sol_algo(SolAlgorithm::AStar)),
            ).run_if(in_state(AppState::Sol).and(is_ready_to_step)));
    }
}

fn is_sol_algo(expected: SolAlgorithm) -> impl FnMut(Res<AlgorithmSelection>) -> bool + Clone {
    move |selection: Res<AlgorithmSelection>| selection.sol_algorithm == expected
}
