use crate::core::{is_ready_to_step, AlgorithmSelection, AppState, Config, GenAlgorithm, Map, MapView, Position, TileState, TileType, TileUpdated};
use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::RngExt;

pub struct MazeGenPlugin;

impl Plugin for MazeGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DFSGenState>()
            .init_resource::<PrimGenState>()
            .init_resource::<KruskalGenState>()
            .add_systems(OnEnter(AppState::Gen), (
                reset_map,
                setup_dfs.run_if(is_gen_algo(GenAlgorithm::DFS)),
                setup_prim.run_if(is_gen_algo(GenAlgorithm::Prim)),
                setup_kruskal.run_if(is_gen_algo(GenAlgorithm::Kruskal))

            ).chain())
            .add_systems(Update, (
                step_dfs.run_if(is_gen_algo(GenAlgorithm::DFS)),
                step_prim.run_if(is_gen_algo(GenAlgorithm::Prim)),
                step_kruskal.run_if(is_gen_algo(GenAlgorithm::Kruskal))
            ).run_if(in_state(AppState::Gen).and(is_ready_to_step)));
    }
}

fn is_gen_algo(expected: GenAlgorithm) -> impl FnMut(Res<AlgorithmSelection>) -> bool + Clone {
    move |selection: Res<AlgorithmSelection>| selection.gen_algorithm == expected
}

#[derive(Resource)]
pub struct DFSGenState {
    pub stack: Vec<Position>,
}

impl Default for DFSGenState {
    fn default() -> Self {
        Self { stack: Vec::new(), }
    }
}

fn setup_dfs(mut state: ResMut<DFSGenState>) {
    state.stack.clear();
    state.stack.push(Position{ x: 1, y: 1 });
}

fn step_dfs(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<DFSGenState>,
    mut next_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    let mut rng = rand::rng();
    if let Some(current) = state.stack.last().copied() {
        let mut unvisited_neighbors = Vec::new();
        for next_pos in map.get_neighbors(&current, 2) {
            if map.get_tile_at_pos(&next_pos) == TileType::Barrier {
                unvisited_neighbors.push(next_pos);
            }

        }
        if !unvisited_neighbors.is_empty() {
            let &next_pos = unvisited_neighbors.choose(&mut rng).unwrap();
            let wall_y = (current.y + next_pos.y) >> 1;
            let wall_x = (current.x + next_pos.x) >> 1;
            map.set_tile(wall_x, wall_y, TileType::Passable(1));
            map.set_tile(next_pos.x, next_pos.y, TileType::Passable(1));
            commands.trigger(TileUpdated {
                entity: map_view.get_entity(wall_x, wall_y),
                state: TileState::Terrain(TileType::Passable(1))
            });
            commands.trigger(TileUpdated {
                entity: map_view.get_entity(next_pos.x, next_pos.y),
                state: TileState::Terrain(TileType::Passable(1))
            });
            state.stack.push(next_pos);
        } else {
            state.stack.pop();
        }
    }else{
        finish_generation(&mut commands, &mut map, &map_view, &mut next_state, &config);
    }
}

#[derive(Resource, Default)]
pub struct PrimGenState {
    pub frontier: Vec<(Position, Position)>,
}

fn setup_prim(mut state: ResMut<PrimGenState>, map: Res<Map>) {
    state.frontier.clear();
    let start = Position::new(1, 1);
    for next_pos in map.get_neighbors(&start, 2) {
        state.frontier.push((Position::new((start.x + next_pos.x)>>1, (start.y + next_pos.y)>>1), next_pos));
    }
}

fn step_prim(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<PrimGenState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    while !state.frontier.is_empty() {
        let idx = rand::random_range(0..state.frontier.len());
        let (wall, next_cell) = state.frontier.swap_remove(idx);
        if map.get_tile_at_pos(&next_cell) == TileType::Barrier {
            map.set_tile(wall.x, wall.y, TileType::Passable(1));
            map.set_tile(next_cell.x, next_cell.y, TileType::Passable(1));
            commands.trigger(TileUpdated { entity: map_view.get_entity(wall.x, wall.y), state: TileState::Terrain(TileType::Passable(1)) });
            commands.trigger(TileUpdated { entity: map_view.get_entity(next_cell.x, next_cell.y), state: TileState::Terrain(TileType::Passable(1)) });
            for next_pos in map.get_neighbors(&next_cell, 2) {
                if map.get_tile_at_pos(&next_pos) == TileType::Barrier {
                    state.frontier.push((Position::new((next_cell.x + next_pos.x)>>1, (next_cell.y + next_pos.y)>>1), next_pos));
                }
            }
            return;
        }
    }
    finish_generation(&mut commands, &mut map, &map_view, &mut next_app_state, &config);
}

#[derive(Resource, Default)]
pub struct KruskalGenState {
    pub walls: Vec<Position>,
    pub parent: HashMap<Position, Position>,
}

fn find(parent: &mut HashMap<Position, Position>, i: Position) -> Position {
    let mut p = *parent.get(&i).unwrap_or(&i);
    if p != i {
        p = find(parent, p);
        parent.insert(i, p);
    }
    p
}

fn setup_kruskal(mut state: ResMut<KruskalGenState>, map: Res<Map>) {
    state.walls.clear();
    state.parent.clear();

    for y in (1..map.height-1).step_by(2) {
        for x in (1..map.width-1).step_by(2) {
            let pos = Position::new(x as i32, y as i32);
            state.parent.insert(pos, pos);
            if x + 2 < map.width { state.walls.push(Position { x:x as i32 + 1, y: y as i32 }); }
            if y + 2 < map.height { state.walls.push(Position { x: x as i32, y: y as i32 + 1 }); }
        }
    }
    let mut rng = rand::rng();
    state.walls.shuffle(&mut rng);
}

fn step_kruskal(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<KruskalGenState>,
    mut next_app_state: ResMut<NextState<AppState>>,
    config: Res<Config>,
) {
    while let Some(wall) = state.walls.pop() {
        let (cell1, cell2) = if wall.x % 2 == 0 {
            (Position::new(wall.x - 1, wall.y), Position::new(wall.x + 1, wall.y))
        } else {
            (Position::new(wall.x, wall.y - 1), Position::new(wall.x, wall.y + 1))
        };
        let root1 = find(&mut state.parent, cell1);
        let root2 = find(&mut state.parent, cell2);
        if root1 != root2 {
            state.parent.insert(root1, root2);
            map.set_tile(wall.x, wall.y, TileType::Passable(1));
            map.set_tile(cell1.x, cell1.y, TileType::Passable(1));
            map.set_tile(cell2.x, cell2.y, TileType::Passable(1));
            commands.trigger(TileUpdated { entity: map_view.get_entity(wall.x, wall.y), state: TileState::Terrain(TileType::Passable(1)) });
            commands.trigger(TileUpdated { entity: map_view.get_entity(cell1.x, cell1.y), state: TileState::Terrain(TileType::Passable(1)) });
            commands.trigger(TileUpdated { entity: map_view.get_entity(cell2.x, cell2.y), state: TileState::Terrain(TileType::Passable(1)) });
            return;
        }
    }
    finish_generation(&mut commands, &mut map, &map_view, &mut next_app_state, &config);
}

fn reset_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
){
    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            if map.get_tile(x, y) != TileType::Barrier {
                map.set_tile(x, y, TileType::Barrier);
                commands.trigger(TileUpdated { entity: map_view.get_entity(x, y), state: TileState::Terrain(TileType::Barrier) });
            }
        }
    }
}

fn finish_generation(
    commands: &mut Commands,
    map: &mut ResMut<Map>,
    map_view: &Res<MapView>,
    next_app_state: &mut ResMut<NextState<AppState>>,
    config: &Res<Config>,
) {
    // randomly make ways that cost more than 1 to passby
    let mut rng = rand::rng();
    let mud_chance = config.mud_chance;

    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            if let TileType::Passable(1) = map.get_tile(x, y) {
                let is_near_start = x < 3 && y < 3;
                let is_near_end = x > (map.width - 4) as i32 && y > (map.height - 4) as i32;
                if !is_near_start && !is_near_end && rng.random_bool(mud_chance) {
                    let weight = rng.random_range(2..=10);
                    map.set_tile(x, y, TileType::Passable(weight));
                    commands.trigger(TileUpdated {
                        entity: map_view.get_entity(x, y),
                        state: TileState::Terrain(map.get_tile(x, y)),
                    });
                }
            }
        }
    }

    // Setup start and end
    map.set_tile(1, 1, TileType::Start);
    commands.trigger(TileUpdated { entity: map_view.get_entity(1, 1), state: TileState::Terrain(TileType::Start) });
    let end_y = (map.height - 2) as i32; let end_x = (map.width - 2) as i32;
    map.set_tile(end_x, end_y, TileType::End);
    commands.trigger(TileUpdated { entity: map_view.get_entity(end_x, end_y), state: TileState::Terrain(TileType::End) });
    next_app_state.set(AppState::Idle);
}
