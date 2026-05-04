use crate::core::{AppState, Map, MapView, StageState, TileType, UpdateTile};
use bevy::app::App;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_ui_button))
            .add_systems(PostStartup, setup_map_ui)
            .add_systems(Update, button_interaction_system);
    }
}

const TILE_SIZE: f32 = 16.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

#[derive(Component)]
struct NextStageButton;

fn setup_ui_button(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(20.0),
            right: px(20.0),
            ..default()
        },
        children![(
            NextStageButton,
            Button,
            Node {
                width: px(200.0),
                height: px(80.0),
                border: UiRect::all(px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            children![(
                Text::new("Waiting..."),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::WHITE),
            )]
        )]
    ));
}

fn setup_map_ui(
    mut commands: Commands,
    map: Res<Map>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_stage_button: ResMut<NextState<StageState>>,
){
    let mut map_view = MapView {
        entities: vec![vec![Entity::PLACEHOLDER; map.width]; map.height],
    };
    let mut observer = Observer::new(on_update_tile);
    for y in 0..map.height {
        for x in 0..map.width {
            let tile_type = map.tiles[y][x];
            let color = get_color_for_tile(tile_type);
            let entity = commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE - 1.0, TILE_SIZE - 1.0)),
                    ..default()
                },
                Transform::from_xyz(
                    (x as f32) * TILE_SIZE - (map.width as f32 * TILE_SIZE / 2.0),
                    (y as f32) * TILE_SIZE - (map.height as f32 * TILE_SIZE / 2.0),
                    0.0,
                )
            )).id();
            map_view.entities[y][x] = entity;
            observer.watch_entity(entity);
        }
    }
    commands.insert_resource(map_view);
    commands.spawn(observer);
}

fn button_interaction_system(
    app_state: Res<State<AppState>>,
    stage_state: Res<State<StageState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_stage_state: ResMut<NextState<StageState>>,
    mut interaction_query: Query<
        (Ref<Interaction>, &mut BackgroundColor, &mut BorderColor, &Children),
        With<NextStageButton>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut bg_color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        let is_idle = *app_state.get() == AppState::Idle;
        let is_finished = *stage_state.get() == StageState::Finished;
        let is_active = is_idle || is_finished;

        let target_text = match app_state.get() {
            AppState::Idle => "Generate maze",
            AppState::Gen => if is_finished { "Solve" } else { "Generating..." },
            AppState::Sol => if is_finished { "Restart" } else { "Solving..." },
        };

        if **text != target_text {
            **text = target_text.to_string();
        }

        if is_active {
            match *interaction {
                Interaction::Pressed => {
                    *bg_color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                    if interaction.is_changed() {
                        match app_state.get() {
                            AppState::Idle => {
                                next_app_state.set(AppState::Gen);
                                next_stage_state.set(StageState::Running);
                            }
                            AppState::Gen => {
                                next_app_state.set(AppState::Sol);
                                next_stage_state.set(StageState::Running);
                            }
                            AppState::Sol => {
                                next_app_state.set(AppState::Idle);
                            }
                        }
                    }
                }
                Interaction::Hovered => {
                    *bg_color = BackgroundColor(Color::srgb(0.3, 0.6, 0.3));
                    *border_color = BorderColor::all(Color::WHITE);
                }
                Interaction::None => {
                    *bg_color = BackgroundColor(Color::srgb(0.2, 0.5, 0.2));
                    *border_color = BorderColor::all(Color::BLACK);
                }
            }
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            *border_color = BorderColor::all(Color::BLACK);
        }
    }
}

fn on_update_tile(
    trigger: On<UpdateTile>,
    mut sprites: Query<&mut Sprite>
) {
    if let Ok(mut sprite) = sprites.get_mut(trigger.entity) {
        sprite.color = get_color_for_tile(trigger.new_type);
    }
}

fn get_color_for_tile(tile_type: TileType) -> Color {
    match tile_type {
        TileType::Barrier => Color::srgb(0.2, 0.2, 0.2),
        TileType::Passable => Color::srgb(0.9, 0.9, 0.9),
        TileType::Start => Color::srgb(0.2, 0.8, 0.2),
        TileType::End => Color::srgb(0.8, 0.2, 0.2),
        TileType::Visited => Color::srgb(0.4, 0.4, 0.4),
        TileType::ShortestPath => Color::srgb(0.8, 0.8, 0.2),
    }
}