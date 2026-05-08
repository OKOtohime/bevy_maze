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
            .add_systems(OnEnter(AppState::Gen), reset_map)
            .add_systems(OnEnter(ActiveGenState(GenAlgorithm::DFS)), setup_dfs)
            .add_systems(OnEnter(ActiveGenState(GenAlgorithm::Prim)), setup_prim)
            .add_systems(OnEnter(ActiveGenState(GenAlgorithm::Kruskal)), setup_kruskal)
            .add_systems(Update, (
                step_gen_algorithm::<DFSGenState>.run_if(in_state(ActiveGenState(GenAlgorithm::DFS))),
                step_gen_algorithm::<PrimGenState>.run_if(in_state(ActiveGenState(GenAlgorithm::Prim))),
                step_gen_algorithm::<KruskalGenState>.run_if(in_state(ActiveGenState(GenAlgorithm::Kruskal)))
            ).run_if(is_ready_to_step));
    }
}
