use std::time::Duration;
use crate::core::{AppState, Map, MapView, Position, TileType, UpdateTile, TIMER_INTERVAL};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::IndexedRandom;

pub struct MazeGenPlugin;

impl Plugin for MazeGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeGenState>()
            .add_systems(OnEnter(AppState::Gen), setup_maze_gen)
            .add_systems(Update, generate_maze_step.run_if(in_state(AppState::Gen).and(on_timer(Duration::from_millis(TIMER_INTERVAL)))));
    }
}

#[derive(Resource)]
pub struct MazeGenState {
    pub stack: Vec<Position>,
}

impl Default for MazeGenState {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
}

fn setup_maze_gen(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<MazeGenState>,
) {
    for y in 0..map.height {
        for x in 0..map.width {
            if map.tiles[y][x] != TileType::Barrier {
                map.tiles[y][x] = TileType::Barrier;
                let entity = map_view.entities[y][x];
                commands.trigger(UpdateTile { entity, new_type: TileType::Barrier });
            }
        }
    }
    state.stack.clear();

    let start = Position { x: 1, y: 1 };
    map.tiles[start.y as usize][start.x as usize] = TileType::Start;
    let entity = map_view.entities[start.y as usize][start.x as usize];
    commands.trigger(UpdateTile { entity, new_type: TileType::Start });
    state.stack.push(start);
}

fn generate_maze_step(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<MazeGenState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut rng = rand::rng();
    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];

    if let Some(current) = state.stack.last().copied() {
        let mut unvisited_neighbors = Vec::new();
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;

            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                if map.tiles[ny as usize][nx as usize] == TileType::Barrier {
                    unvisited_neighbors.push((nx, ny, dx / 2, dy / 2));
                }
            }
        }

        if !unvisited_neighbors.is_empty() {
            let &(nx, ny, wx, wy) = unvisited_neighbors.choose(&mut rng).unwrap();

            let wall_y = (current.y + wy) as usize;
            let wall_x = (current.x + wx) as usize;
            let next_y = ny as usize;
            let next_x = nx as usize;

            map.tiles[wall_y][wall_x] = TileType::Passable;
            map.tiles[next_y][next_x] = TileType::Passable;

            commands.trigger(UpdateTile {
                entity: map_view.entities[wall_y][wall_x],
                new_type: TileType::Passable,
            });
            commands.trigger(UpdateTile {
                entity: map_view.entities[next_y][next_x],
                new_type: TileType::Passable,
            });

            state.stack.push(Position { x: nx, y: ny });
        } else {
            state.stack.pop();
        }
    } else {
        let end_y = map.height - 2;
        let end_x = map.width - 2;

        map.tiles[end_y][end_x] = TileType::End;
        commands.trigger(UpdateTile {
            entity: map_view.entities[end_y][end_x],
            new_type: TileType::End,
        });
        next_state.set(AppState::Idle);
    }
}
