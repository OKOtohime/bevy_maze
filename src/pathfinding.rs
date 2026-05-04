use crate::core::{AppState, Map, MapView, Position, StageState, TileType, UpdateTile};
use crate::generation::MazeGenState;
use bevy::app::App;
use bevy::platform::collections::HashMap;
use bevy::prelude::{in_state, Commands, IntoScheduleConfigs, NextState, OnEnter, Plugin, Res, ResMut, Resource, SystemCondition, Time, Timer, TimerMode, Update};
use std::collections::VecDeque;

pub struct MazeSolPlugin;

impl Plugin for MazeSolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeSolState>()
            .add_systems(OnEnter(AppState::Sol), setup_maze_sol)
            .add_systems(Update, solve_maze_step.run_if(in_state(AppState::Sol).and(in_state(StageState::Running))));
    }
}

#[derive(Resource)]
pub struct MazeSolState {
    pub queue: VecDeque<Position>,
    pub came_from: HashMap<Position, Position>,
    pub backtrack: Option<Position>,
    pub timer: Timer,
}

impl Default for MazeSolState {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
            came_from: HashMap::new(),
            backtrack: None,
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
        }
    }
}

fn setup_maze_sol(mut state: ResMut<MazeSolState>) {
    state.queue.clear();
    state.came_from.clear();
    state.backtrack = None;
    
    let start_pos = Position { x: 1, y: 1 };
    state.queue.push_back(start_pos);
    state.came_from.insert(start_pos, start_pos);
}

// BFS
fn solve_maze_step(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut state: ResMut<MazeSolState>,
    mut next_stage_state: ResMut<NextState<StageState>>,
    time: Res<Time>,
) {
    state.timer.tick(time.delta());
    if !state.timer.just_finished() { return; }

    // the shortest path is found, draw the line
    if let Some(current_backtrack) = state.backtrack {
        if let Some(&parent) = state.came_from.get(&current_backtrack) {
            if parent == (Position{x:1, y:1}){
                state.backtrack = None;
                next_stage_state.set(StageState::Finished);
                return;
            }
            let entity = map_view.entities[parent.y as usize][parent.x as usize];
            map.tiles[parent.y as usize][parent.x as usize] = TileType::ShortestPath;
            commands.trigger(UpdateTile { entity, new_type: TileType::ShortestPath });
            state.backtrack = Some(parent);
        }
        return;
    }

    // search path
    if let Some(current) = state.queue.pop_front() {
        if current.y == (map.height - 2) as i32 && current.x == (map.width - 2) as i32 {
            state.backtrack = Some(current);
            state.queue.clear();
            return;
        }
        let entity = map_view.entities[current.y as usize][current.x as usize];
        if map.tiles[current.y as usize][current.x as usize] == TileType::Passable {
            map.tiles[current.y as usize][current.x as usize] = TileType::Visited;
            commands.trigger(UpdateTile{entity, new_type: TileType::Visited});
        }
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = current.x + dx;
            let ny = current.y + dy;
            if nx > 0 && nx < (map.width - 1) as i32 && ny > 0 && ny < (map.height - 1) as i32 {
                let next_pos = Position{x: nx, y: ny};
                let target_tile = map.tiles[ny as usize][nx as usize];
                if target_tile == TileType::Passable || target_tile == TileType::End {
                    if !state.came_from.contains_key(&next_pos) {
                        state.queue.push_back(Position { x: nx, y: ny });
                        state.came_from.insert(next_pos, current);
                    }
                }
            }
        }
    }else{
        next_stage_state.set(StageState::Finished);
    }
}
