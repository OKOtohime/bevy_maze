use std::time::Duration;
use bevy::prelude::*;

// To visualize the algorithm process, we have to run the algorithm step by step
#[derive(Resource)]
pub struct Config {
    pub step_timer: Timer,
    pub mud_chance: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            step_timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
            mud_chance: 0.05,
        }
    }
}

pub fn tick_step_timer(time: Res<Time>, mut config: ResMut<Config>) {
    config.step_timer.tick(time.delta());
}

pub fn is_ready_to_step(config: Res<Config>) -> bool {
    config.step_timer.just_finished()
}
