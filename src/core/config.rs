use std::time::Duration;
use bevy::prelude::*;

// To visualize the algorithm process, we have to run the algorithm step by step
#[derive(Resource)]
pub struct Config {
    pub step_timer: Timer,
    pub speed_multiplier: u32, // step multiplier steps per frame
    pub mud_chance: f64,
    pub maze_width: usize,
    pub maze_height: usize,
    pub start_pos: IVec2,
    pub end_pos: IVec2,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            step_timer: Timer::new(Duration::from_millis(10), TimerMode::Repeating),
            speed_multiplier: 1,
            mud_chance: 0.0,
            maze_width: 20,
            maze_height: 20,
            start_pos: IVec2::new(1, 1),
            end_pos: IVec2::new(40, 40),
        }
    }
}

pub fn tick_step_timer(time: Res<Time>, mut config: ResMut<Config>) {
    config.step_timer.tick(time.delta());
}

pub fn is_ready_to_step(config: Res<Config>) -> bool {
    config.step_timer.just_finished()
}
