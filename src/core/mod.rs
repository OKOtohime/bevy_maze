pub mod config;
pub mod state;
pub mod grid;
pub mod map;
pub mod prelude;

use bevy::prelude::*;
use crate::core::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<GenAlgorithm>()
            .init_state::<SolAlgorithm>()
            .add_computed_state::<ActiveGenState>()
            .add_computed_state::<ActiveSolState>()
            .init_resource::<Config>()
            .add_message::<GenerationFinished>()
            .add_message::<PathfindingFinished>()
            .insert_resource(Map::new_maze(20, 20))
            .add_systems(Update, tick_step_timer)
            .add_systems(Update, handle_algorithm_finished);
    }
}
