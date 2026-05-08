use crate::core::prelude::*;
use crate::ui::common::{MapResize, MapSyncEndpoints};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn config_panel_system(
    mut contexts: EguiContexts,
    mut config: ResMut<Config>,
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
    registry: Res<AlgorithmRegistry>,
    mut selection: ResMut<SelectedAlgorithms>,
    mut ev_resize: MessageWriter<MapResize>,
    mut ev_sync_pos: MessageWriter<MapSyncEndpoints>,
) {
    let is_idle = *app_state.get() == AppState::Idle;

    let mut size_changed = false;
    let mut pos_changed = false;

    let old_start = config.start_pos;
    let old_end = config.end_pos;

    let ui_max_x = ((config.maze_width << 1) - 1) as i32;
    let ui_max_y = ((config.maze_height << 1) - 1) as i32;

    egui::SidePanel::left("settings_panel")
        .default_width(220.0)
        .resizable(false)
        .show(contexts.ctx_mut().expect("Failed to get Egui context"), |ui| {

            ui.add_space(10.0);
            ui.heading("Dimensions");
            ui.horizontal(|ui| {
                ui.add_enabled_ui(is_idle, |ui| {
                    ui.label("Width:");
                    if ui.add(egui::DragValue::new(&mut config.maze_width).range(10..=50)).changed() {
                        size_changed = true;
                    }
                    ui.label("Height:");
                    if ui.add(egui::DragValue::new(&mut config.maze_height).range(10..=50)).changed() {
                        size_changed = true;
                    }
                });
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut config.speed_multiplier, 1..=50).text("steps/tick"));
            });

            ui.separator();

            ui.heading("Generation");
            ui.horizontal(|ui| {
                ui.label("Mud Chance:");
                ui.add(egui::Slider::new(&mut config.mud_chance, 0.0..=1.0).text("%"));
            });

            ui.separator();

            ui.heading("Positions");
            ui.add_enabled_ui(is_idle, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Start (X, Y):");
                    if ui.add(egui::DragValue::new(&mut config.start_pos.x).range(1..=ui_max_x)).changed() {pos_changed = true;}
                    if ui.add(egui::DragValue::new(&mut config.start_pos.y).range(1..=ui_max_y)).changed() {pos_changed = true;}
                });
                ui.horizontal(|ui| {
                    ui.label("End (X, Y):");
                    if ui.add(egui::DragValue::new(&mut config.end_pos.x).range(1..=ui_max_x)).changed() {pos_changed = true;}
                    if ui.add(egui::DragValue::new(&mut config.end_pos.y).range(1..=ui_max_y)).changed() {pos_changed = true;}
                });
            });

            ui.separator();

            ui.heading("Algorithms");

            egui::ComboBox::from_label("Generator")
                .selected_text(selection.gen_algorithm)
                .show_ui(ui, |ui| {
                    ui.add_enabled_ui(is_idle, |ui| {
                        for &algo_name in &registry.generators {
                            ui.selectable_value(&mut selection.gen_algorithm, algo_name, algo_name);
                        }
                    });
                });
            ui.add_space(5.0);
            egui::ComboBox::from_label("Solver")
                .selected_text(selection.sol_algorithm)
                .show_ui(ui, |ui| {
                    ui.add_enabled_ui(is_idle, |ui| {
                        for &algo_name in &registry.solvers {
                            ui.selectable_value(&mut selection.sol_algorithm, algo_name, algo_name);
                        }
                    });
                });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add_enabled_ui(is_idle, |ui| {
                    if ui.button("Generate").clicked() {
                        next_app_state.set(AppState::Gen);
                    }
                    if ui.button("Solve").clicked() {
                        next_app_state.set(AppState::Sol);
                    }
                });
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                ui.label("Powered by Bevy & Egui");
            });
        });

    let real_max_x = ((config.maze_width << 1) - 1) as i32;
    let real_max_y = ((config.maze_height << 1) - 1) as i32;

    let force_odd = |val: i32| if val % 2 == 0 { val - 1 } else { val };
    let clamped_start_x = config.start_pos.x.clamp(1, real_max_x);
    let clamped_start_y = config.start_pos.y.clamp(1, real_max_y);
    let clamped_end_x = config.end_pos.x.clamp(1, real_max_x);
    let clamped_end_y = config.end_pos.y.clamp(1, real_max_y);

    config.start_pos.x = force_odd(clamped_start_x);
    config.start_pos.y = force_odd(clamped_start_y);
    config.end_pos.x = force_odd(clamped_end_x);
    config.end_pos.y = force_odd(clamped_end_y);

    if config.start_pos != old_start || config.end_pos != old_end {
        pos_changed = true;
    }

    if size_changed {
        ev_resize.write(MapResize);
    } else if pos_changed {
        ev_sync_pos.write(MapSyncEndpoints);
    }
}