mod core;
mod generation;
mod pathfinding;
mod ui;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::WindowMode;
use crate::core::CorePlugin;
use crate::generation::MazeGenPlugin;
use crate::pathfinding::MazeSolPlugin;
use crate::ui::{UIPlugin, DESKTOP_HEIGHT, DESKTOP_WIDTH};

pub struct MazePlugin;

impl PluginGroup for MazePlugin {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin{width: 35, height: 32})
            .add(MazeGenPlugin)
            .add(MazeSolPlugin)
            .add(UIPlugin)
    }
}

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
            MazePlugin
        )).run();
}
