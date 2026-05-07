use crate::core::{AlgorithmSelection, AppState, GenAlgorithm, Map, MapView, SolAlgorithm, TileState, TileType, TileUpdated};
use bevy::app::App;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_ui))
            .add_systems(PostStartup, setup_map_ui)
            .add_systems(Update, (control_button_system, alg_select_system));
    }
}

pub const DESKTOP_WIDTH: u32 = 1280;
pub const DESKTOP_HEIGHT: u32 = 1024;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

#[derive(Component)]
struct GenerateBtn;

#[derive(Component)]
struct SolveBtn;

#[derive(Component)]
pub struct GenSelectBtn(pub GenAlgorithm);

#[derive(Component)]
pub struct SolSelectBtn(pub SolAlgorithm);

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(20.0),
            top: px(20.0),
            flex_direction: FlexDirection::Column,
            row_gap: px(15.0),
            ..default()
        },
    )).with_children(|parent| {
        spawn_label(parent, "Maze generator");
        spawn_btn_with_component(parent, "DFS", GenSelectBtn(GenAlgorithm::DFS));
        spawn_btn_with_component(parent, "Prim", GenSelectBtn(GenAlgorithm::Prim));
        spawn_btn_with_component(parent, "Kruskal", GenSelectBtn(GenAlgorithm::Kruskal));

        spawn_label(parent, "Path Finder");
        spawn_btn_with_component(parent, "BFS", SolSelectBtn(SolAlgorithm::BFS));
        spawn_btn_with_component(parent, "Dijkstra", SolSelectBtn(SolAlgorithm::Dijkstra));
        spawn_btn_with_component(parent, "A* Search", SolSelectBtn(SolAlgorithm::AStar));

        spawn_label(parent, "Control");
        spawn_btn_with_component(parent, "Generate", GenerateBtn);
        spawn_btn_with_component(parent, "Solve", SolveBtn);
    });
}

fn spawn_label(parent: &mut ChildSpawnerCommands, text: &str){
    parent.spawn((
        Node { margin: UiRect::top(px(20.0)), ..default() },
        Text::new(text),
        TextFont { font_size: 20.0, ..default() },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
    ));
}

fn spawn_btn_with_component(parent: &mut ChildSpawnerCommands, text: &str, component: impl Component) {
    parent.spawn((
        component,
        Button,
        Node {
            width: px(180.0),
            height: px(40.0),
            border: UiRect::all(px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    )).with_children(|btn| {
        btn.spawn((
            Text::new(text),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

fn control_button_system(
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut gen_btn_query: Query<(Ref<Interaction>, &mut BackgroundColor, &mut BorderColor), (With<GenerateBtn>, Without<SolveBtn>)>,
    mut sol_btn_query: Query<(Ref<Interaction>, &mut BackgroundColor, &mut BorderColor), (With<SolveBtn>, Without<GenerateBtn>)>,
) {
    let is_idle = *app_state.get() == AppState::Idle;
    let mut handle_interaction = |interaction: Ref<Interaction>, bg: &mut BackgroundColor, border: &mut BorderColor, target_state: AppState| {
        if is_idle {
            match *interaction {
                Interaction::Pressed => {
                    *bg = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                    if interaction.is_changed() { next_app_state.set(target_state); }
                }
                Interaction::Hovered => {
                    *bg = BackgroundColor(Color::srgb(0.3, 0.6, 0.3));
                    *border = BorderColor::all(Color::WHITE);
                }
                Interaction::None => {
                    *bg = BackgroundColor(Color::srgb(0.2, 0.5, 0.2));
                    *border = BorderColor::all(Color::BLACK);
                }
            }
        } else {
            *bg = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            *border = BorderColor::all(Color::BLACK);
        }
    };
    for (int, mut bg, mut border) in &mut gen_btn_query { handle_interaction(int, &mut bg, &mut border, AppState::Gen); }
    for (int, mut bg, mut border) in &mut sol_btn_query { handle_interaction(int, &mut bg, &mut border, AppState::Sol); }
}

fn alg_select_system(
    mut selection: ResMut<AlgorithmSelection>,
    app_state: Res<State<AppState>>,
    mut gen_btns: Query<(Ref<Interaction>, &GenSelectBtn, &mut BackgroundColor, &mut BorderColor), Without<SolSelectBtn>>,
    mut sol_btns: Query<(Ref<Interaction>, &SolSelectBtn, &mut BackgroundColor, &mut BorderColor), Without<GenSelectBtn>>,
) {
    let can_switch = *app_state.get() == AppState::Idle;
    for (interaction, btn_type, mut bg, mut border) in &mut gen_btns {
        let is_selected = selection.gen_algorithm == btn_type.0;
        if can_switch && *interaction == Interaction::Pressed {
            selection.gen_algorithm = btn_type.0;
        }
        update_radio_btn_color(*interaction, is_selected, can_switch, &mut bg, &mut border);
    }

    for (interaction, btn_type, mut bg, mut border) in &mut sol_btns {
        let is_selected = selection.sol_algorithm == btn_type.0;
        if can_switch && *interaction == Interaction::Pressed {
            selection.sol_algorithm = btn_type.0;
        }
        update_radio_btn_color(*interaction, is_selected, can_switch, &mut bg, &mut border);
    }
}

fn update_radio_btn_color(
    interaction: Interaction,
    is_selected: bool,
    can_switch: bool,
    bg: &mut BackgroundColor,
    border: &mut BorderColor
) {
    if is_selected {
        *bg = BackgroundColor(Color::srgb(0.2, 0.5, 0.8));
        *border = BorderColor::all(Color::WHITE);
    } else if can_switch {
        match interaction {
            Interaction::Hovered => *bg = BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
            _ => *bg = BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        }
        *border = BorderColor::all(Color::BLACK);
    } else {
        *bg = BackgroundColor(Color::srgb(0.1, 0.1, 0.1));
        *border = BorderColor::all(Color::BLACK);
    }
}

fn setup_map_ui(
    mut commands: Commands,
    map: Res<Map>,
){
    let mut map_view = MapView {
        width: map.width,
        height: map.height,
        entities: vec![Entity::PLACEHOLDER; map.width * map.height],
    };
    let mut observer = Observer::new(on_paint_tile);

    let ui_pannel_width = 200.0;
    let screen_padding = 40.0;

    let usable_width = DESKTOP_WIDTH as f32 - ui_pannel_width - screen_padding;
    let usable_height = DESKTOP_HEIGHT as f32 - screen_padding;

    let tile_size = (usable_width / map.width as f32).min(usable_height / map.height as f32).floor();
    let map_pixel_width = map.width as f32 * tile_size;
    let map_pixel_height = map.height as f32 * tile_size;

    let offset_x = -(DESKTOP_WIDTH as f32 / 2.0) + ui_pannel_width + (usable_width / 2.0);
    let offset_y = 0.0;

    for y in 0..map.height as i32{
        for x in 0..map.width as i32 {
            let tile_type = map.get_tile(x, y);
            let color = get_color_for_state(TileState::Terrain(tile_type));
            let base_x = (x as f32) * tile_size - (map_pixel_width / 2.0) + (tile_size / 2.0);
            let base_y = (y as f32) * tile_size - (map_pixel_height / 2.0) + (tile_size / 2.0);
            let entity = commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new((tile_size - 1.0).max(1.0), (tile_size - 1.0).max(1.0))),
                    ..default()
                },
                Transform::from_xyz(
                    base_x + offset_x,
                    base_y + offset_y,
                    0.0,
                )
            )).id();
            map_view.set_entity(x, y, entity);
            observer.watch_entity(entity);
        }
    }
    commands.insert_resource(map_view);
    commands.spawn(observer);
}

const COLOR_BARRIER: Color = Color::srgb(0.2, 0.2, 0.2);
const COLOR_PASSIBLE: Color = Color::srgb(0.9, 0.9, 0.9);
const COLOR_START: Color = Color::srgb(0.2, 0.8, 0.2);
const COLOR_END: Color = Color::srgb(0.8, 0.2, 0.2);
const COLOR_VISITED: Color = Color::srgb(0.4, 0.4, 0.4);
const COLOR_PATH: Color = Color::srgb(0.8, 0.8, 0.2);

fn on_paint_tile(
    trigger: On<TileUpdated>,
    mut sprites: Query<&mut Sprite>
) {
    if let Ok(mut sprite) = sprites.get_mut(trigger.entity) {
        sprite.color = get_color_for_state(trigger.state);
    }
}

pub fn get_color_for_state(state: TileState) -> Color {
    match state {
        TileState::Terrain(tile_type) => match tile_type {
            TileType::Barrier => COLOR_BARRIER,
            TileType::Passable(cost) => {
                if cost == 1 {
                    COLOR_PASSIBLE
                }else{
                    let t = (cost as f32 - 2.0) / 8.0;
                    let r_start = 0.8; let g_start = 0.7; let b_start = 0.5;
                    let r_end = 0.3; let g_end = 0.15; let b_end = 0.05;
                    let r = r_start * (1.0 - t) + r_end * t;
                    let g = g_start * (1.0 - t) + g_end * t;
                    let b = b_start * (1.0 - t) + b_end * t;
                    Color::srgb(r, g, b)
                }
            },
            TileType::Start => COLOR_START,
            TileType::End => COLOR_END,
        },
        TileState::Visited => COLOR_VISITED,
        TileState::Path => COLOR_PATH,
    }
}