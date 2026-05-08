use crate::core::prelude::*;
use bevy::prelude::*;
use rand::RngExt;

pub enum GenStepResult {
    TilesModified(Vec<IVec2>),  // Made new passible tile
    InProgress, // Nothing to do
    Finished    // finish generation
}

pub trait SteppedGenAlgorithm {
    fn step(&mut self, map: &Map, config: &Config) -> GenStepResult;
}

pub fn step_gen_algorithm<T: SteppedGenAlgorithm + Resource>(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
    mut algo_state: ResMut<T>,
    mut ev_finished: MessageWriter<GenerationFinished>,
    config: Res<Config>,
){
    for _ in 0..config.speed_multiplier {
        match algo_state.step(&map, &config) {
            GenStepResult::TilesModified(tiles) => {
                for pos in tiles{
                    update_map_at_pos(&mut commands, &mut map, &map_view, pos, TileType::Passable(1));
                }
            }
            GenStepResult::InProgress => {}
            GenStepResult::Finished => {
                finish_generation(&mut commands, &mut map, &map_view, &mut ev_finished, &config);
            }
        }
    }
}

// Wrap map tile update with trigger
pub fn update_map_at_pos(
    commands: &mut Commands,
    map: &mut Map,
    map_view: &MapView,
    pos: IVec2,
    tile_type: TileType,
) {
    map.set(pos.x, pos.y, tile_type);
    commands.trigger(TileUpdated {
        entity: *map_view.get(pos.x, pos.y),
        state: TileState::Terrain(tile_type),
    });
}

pub fn reset_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    map_view: Res<MapView>,
){
    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            if *map.get(x, y) != TileType::Barrier {
                update_map_at_pos(&mut commands, &mut map, &map_view, IVec2{x, y}, TileType::Barrier);
            }
        }
    }
}

pub fn finish_generation(
    commands: &mut Commands,
    map: &mut Map,
    map_view: &MapView,
    ev_finished: &mut MessageWriter<GenerationFinished>,
    config: &Config,
) {
    // randomly make ways that cost more than 1 to passby
    let mut rng = rand::rng();
    let mud_chance = config.mud_chance;

    for y in 0..map.height as i32{
        for x in 0..map.width as i32{
            if let TileType::Passable(1) = map.get(x, y) {
                let is_near_start = x < 3 && y < 3;
                let is_near_end = x > (map.width - 4) as i32 && y > (map.height - 4) as i32;
                if !is_near_start && !is_near_end && rng.random_bool(mud_chance) {
                    let weight = rng.random_range(2..=10);
                    update_map_at_pos(commands, map, &map_view, IVec2{x, y}, TileType::Passable(weight));
                }
            }
        }
    }

    // Setup start and end
    update_map_at_pos(commands, map, &map_view, config.start_pos, TileType::Start);
    update_map_at_pos(commands, map, &map_view, config.end_pos, TileType::End);
    ev_finished.write(GenerationFinished);
}