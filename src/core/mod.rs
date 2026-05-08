pub mod config;
pub mod state;
pub mod grid;
pub mod map;
pub mod prelude;

use bevy::prelude::*;
use crate::core::prelude::*;

pub struct CorePlugin {
    pub width: usize,
    pub height: usize,
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<AlgorithmSelection>()
            .init_resource::<Config>()
            .insert_resource(Map::new_maze(self.width, self.height))
            .add_systems(Update, tick_step_timer);
    }
}

impl Default for CorePlugin {
    fn default() -> Self {
        Self{ width: 20, height: 20 }
    }
}
