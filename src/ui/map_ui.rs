use bevy::prelude::*;
use crate::core::prelude::*;
use crate::generation::common::update_map_at_pos;
use super::common::*;
use super::{DESKTOP_WIDTH, DESKTOP_HEIGHT};

#[derive(Component)]
pub struct TileEntity;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn setup_map_ui(
    mut commands: Commands,
    map: Res<Map>,
){
    let map_view = spawn_map_sprites(&mut commands, &map);
    commands.insert_resource(map_view);
    commands.spawn(Observer::new(on_paint_tile));
}

fn spawn_map_sprites(commands: &mut Commands, map: &Map) -> MapView {
    let mut map_view = MapView {
        width: map.width,
        height: map.height,
        data: vec![Entity::PLACEHOLDER; map.width * map.height],
    };
    let ui_pannel_width = 280.0;
    let screen_padding = 40.0;
    let usable_width = DESKTOP_WIDTH as f32 - ui_pannel_width - screen_padding;
    let usable_height = DESKTOP_HEIGHT as f32 - screen_padding;

    let tile_size = (usable_width / map.width as f32).min(usable_height / map.height as f32).floor();
    let map_pixel_width = map.width as f32 * tile_size;
    let map_pixel_height = map.height as f32 * tile_size;
    let offset_x = -(DESKTOP_WIDTH as f32 / 2.0) + ui_pannel_width + (usable_width / 2.0);

    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            let tile_type = *map.get(x, y);
            let base_x = (x as f32) * tile_size - (map_pixel_width / 2.0) + (tile_size / 2.0);
            let base_y = (y as f32) * tile_size - (map_pixel_height / 2.0) + (tile_size / 2.0);
            let entity = commands.spawn((
                TileEntity,
                Sprite {
                    color: get_color_for_state(TileState::Terrain(tile_type)),
                    custom_size: Some(Vec2::new((tile_size - 1.0).max(1.0), (tile_size - 1.0).max(1.0))),
                    ..default()
                },
                Transform::from_xyz(base_x + offset_x, base_y, 0.0)
            )).id();

            map_view.set(x, y, entity);
        }
    }
    map_view
}

pub fn handle_map_resize(
    mut ev_resize: MessageReader<MapResize>,
    mut commands: Commands,
    config: Res<Config>,
    tiles_query: Query<Entity, With<TileEntity>>,
) {
    if ev_resize.read().next().is_some() {
        for entity in &tiles_query {
            commands.entity(entity).despawn();
        }
        let mut new_map = Map::new_maze(config.maze_width, config.maze_height);

        new_map.set_at_pos(&config.start_pos, TileType::Start);
        new_map.set_at_pos(&config.end_pos, TileType::End);

        let new_map_view = spawn_map_sprites(&mut commands, &new_map);

        commands.insert_resource(new_map);
        commands.insert_resource(new_map_view);
    }
}

pub fn handle_sync_endpoints(
    mut ev_sync: MessageReader<MapSyncEndpoints>,
    mut commands: Commands,
    config: Res<Config>,
    mut map: ResMut<Map>,
    map_view: Option<Res<MapView>>,
) {
    if ev_sync.read().next().is_some() {
        let Some(view) = map_view else { return };

        for y in 0..map.height as i32 {
            for x in 0..map.width as i32 {
                let current_tile = *map.get(x, y);
                if current_tile == TileType::Start || current_tile == TileType::End {
                    update_map_at_pos(&mut commands, &mut map, &view, IVec2::new(x, y), TileType::Passable(1));
                }
            }
        }

        update_map_at_pos(&mut commands, &mut map, &view, config.start_pos, TileType::Start);
        update_map_at_pos(&mut commands, &mut map, &view, config.end_pos, TileType::End);
    }
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
