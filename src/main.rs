mod core;
mod generation;
mod pathfinding;
mod ui;

use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::core::CorePlugin;
use crate::generation::MazeGenPlugin;
use crate::pathfinding::MazeSolPlugin;
use crate::ui::UIPlugin;

const DESKTOP_WIDTH: u32 = 1024;
const DESKTOP_HEIGHT: u32 = 1024;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Maze".into(),
                    resolution: (DESKTOP_WIDTH, DESKTOP_HEIGHT).into(),
                    mode: WindowMode::Windowed,
                    resizable: false,
                    visible: true,
                    ..default()
                }), ..default()
            }),
            CorePlugin{width: 20, height: 20},
            MazeGenPlugin,
            MazeSolPlugin,
            UIPlugin
        )).run();
}
