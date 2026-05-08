pub mod common;
pub mod dfs;
pub mod prim;
pub mod kruskal;
pub mod prelude;

use crate::core::prelude::*;
use bevy::prelude::*;
use prelude::*;

pub struct MazeGenPlugin;

impl Plugin for MazeGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DFSGenState>()
            .init_resource::<PrimGenState>()
            .init_resource::<KruskalGenState>()
            .add_systems(OnEnter(AppState::Gen), reset_map)
            .add_plugins((
                DFSPlugin,
                PrimPlugin,
                KruskalPlugin,
            ));
    }
}
