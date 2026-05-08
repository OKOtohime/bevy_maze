pub mod config_ui;
pub mod map_ui;
mod common;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use map_ui::*;
use config_ui::*;
use crate::ui::common::{MapResize, MapSyncEndpoints};

pub const DESKTOP_WIDTH: u32 = 1280;
pub const DESKTOP_HEIGHT: u32 = 1024;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_message::<MapResize>()
            .add_message::<MapSyncEndpoints>()
            .add_systems(Startup, setup_camera)
            .add_systems(PostStartup, setup_map_ui)
            .add_systems(EguiPrimaryContextPass, config_panel_system)
            .add_systems(Update, (handle_map_resize, handle_sync_endpoints));
    }
}
