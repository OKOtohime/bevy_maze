pub mod common;
pub mod dfs;
pub mod prim;
pub mod kruskal;
pub mod prelude;

use bevy::prelude::*;
use crate::core::prelude::*;
use prelude::*;

pub struct MazeGenPlugin;

impl Plugin for MazeGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DFSGenState>()
            .init_resource::<PrimGenState>()
            .init_resource::<KruskalGenState>()
            .add_systems(OnEnter(AppState::Gen), (
                reset_map,
                setup_dfs.run_if(is_gen_algo(GenAlgorithm::DFS)),
                setup_prim.run_if(is_gen_algo(GenAlgorithm::Prim)),
                setup_kruskal.run_if(is_gen_algo(GenAlgorithm::Kruskal))

            ).chain())
            .add_systems(Update, (
                step_dfs.run_if(is_gen_algo(GenAlgorithm::DFS)),
                step_prim.run_if(is_gen_algo(GenAlgorithm::Prim)),
                step_kruskal.run_if(is_gen_algo(GenAlgorithm::Kruskal))
            ).run_if(in_state(AppState::Gen).and(is_ready_to_step)));
    }
}

fn is_gen_algo(expected: GenAlgorithm) -> impl FnMut(Res<AlgorithmSelection>) -> bool + Clone {
    move |selection: Res<AlgorithmSelection>| selection.gen_algorithm == expected
}
