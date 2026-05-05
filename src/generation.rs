use std::time::Duration;
use crate::core::{AlgorithmSelection, AppState, GenAlgorithm, Map, MapView, PaintTile, Position, TileType, TIMER_INTERVAL};
use bevy::app::{App, Plugin};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::RngExt;
use crate::ui::{get_color_for_tile, COLOR_BARRIER, COLOR_END, COLOR_PASSIBLE, COLOR_START};

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
            ).run_if(in_state(AppState::Gen).and(on_timer(Duration::from_millis(TIMER_INTERVAL)))));
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
            map.tiles[wall_y][wall_x] = TileType::Passable(1);
            map.tiles[next_y][next_x] = TileType::Passable(1);
            commands.trigger(PaintTile {
                entity: map_view.entities[wall_y][wall_x],
                color: COLOR_PASSIBLE
            });
            commands.trigger(PaintTile {
                entity: map_view.entities[next_y][next_x],
                color: COLOR_PASSIBLE
            });
            state.stack.push(Position { x: nx, y: ny });
        } else {
            state.stack.pop();
        }
    }else{
        finish_generation(&mut commands, &mut map, &map_view, &mut next_state);
    }
}

#[derive(Resource, Default)]
pub struct PrimGenState {
    pub frontier: Vec<(Position, Position)>,
}

fn setup_prim(mut state: ResMut<PrimGenState>, map: Res<Map>) {
    state.frontier.clear();
    let start = Position::new(1, 1);
    let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
    for (dx, dy) in directions.iter() {
        let nx = start.x + dx; let ny = start.y + dy;
        if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
            state.frontier.push((Position::new(start.x + dx/2, start.y + dy/2), Position::new(nx, ny)));
        }
    }
}

fn step_prim(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<PrimGenState>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    while !state.frontier.is_empty() {
        let idx = rand::random_range(0..state.frontier.len());
        let (wall, next_cell) = state.frontier.swap_remove(idx);
        if map.tiles[next_cell.y as usize][next_cell.x as usize] == TileType::Barrier {
            map.tiles[wall.y as usize][wall.x as usize] = TileType::Passable(1);
            map.tiles[next_cell.y as usize][next_cell.x as usize] = TileType::Passable(1);
            commands.trigger(PaintTile { entity: map_view.entities[wall.y as usize][wall.x as usize], color: COLOR_PASSIBLE });
            commands.trigger(PaintTile { entity: map_view.entities[next_cell.y as usize][next_cell.x as usize], color: COLOR_PASSIBLE });
            let directions = [(0, 2), (2, 0), (0, -2), (-2, 0)];
            for (dx, dy) in directions.iter() {
                let nx = next_cell.x + dx;
                let ny = next_cell.y + dy;
                if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                    if map.tiles[ny as usize][nx as usize] == TileType::Barrier {
                        state.frontier.push((Position::new(next_cell.x + dx/2, next_cell.y + dy/2), Position::new(nx, ny)));
                    }
                }
            }
            return;
        }
    }
    finish_generation(&mut commands, &mut map, &map_view, &mut next_app_state);
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
            map.tiles[wall.y as usize][wall.x as usize] = TileType::Passable(1);
            map.tiles[cell1.y as usize][cell1.x as usize] = TileType::Passable(1);
            map.tiles[cell2.y as usize][cell2.x as usize] = TileType::Passable(1);
            commands.trigger(PaintTile { entity: map_view.entities[wall.y as usize][wall.x as usize], color: COLOR_PASSIBLE });
            commands.trigger(PaintTile { entity: map_view.entities[cell1.y as usize][cell1.x as usize], color: COLOR_PASSIBLE });
            commands.trigger(PaintTile { entity: map_view.entities[cell2.y as usize][cell2.x as usize], color: COLOR_PASSIBLE });
            return;
        }
    }
    finish_generation(&mut commands, &mut map, &map_view, &mut next_app_state);
}

fn reset_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
){
    for y in 0..map.height {
        for x in 0..map.width {
            if map.tiles[y][x] != TileType::Barrier {
                map.tiles[y][x] = TileType::Barrier;
                commands.trigger(PaintTile { entity: map_view.entities[y][x], color: COLOR_BARRIER });
            }
        }
    }
}

fn finish_generation(
    commands: &mut Commands,
    map: &mut ResMut<Map>,
    map_view: &Res<MapView>,
    next_app_state: &mut ResMut<NextState<AppState>>
) {
    // randomly make ways that cost more than 1 to passby
    let mut rng = rand::rng();
    let mud_chance = 0.05;

    for y in 0..map.height {
        for x in 0..map.width {
            if let TileType::Passable(1) = map.tiles[y][x] {
                let is_near_start = x < 3 && y < 3;
                let is_near_end = x > map.width - 4 && y > map.height - 4;
                if !is_near_start && !is_near_end && rng.random_bool(mud_chance) {
                    let weight = rng.random_range(2..=10);
                    map.tiles[y][x] = TileType::Passable(weight);
                    commands.trigger(PaintTile {
                        entity: map_view.entities[y][x],
                        color: get_color_for_tile(map.tiles[y][x])
                    });
                }
            }
        }
    }

    // Setup start and end
    map.tiles[1][1] = TileType::Start;
    commands.trigger(PaintTile { entity: map_view.entities[1][1], color: COLOR_START });
    let end_y = map.height - 2; let end_x = map.width - 2;
    map.tiles[end_y][end_x] = TileType::End;
    commands.trigger(PaintTile { entity: map_view.entities[end_y][end_x], color: COLOR_END });
    next_app_state.set(AppState::Idle);
}
