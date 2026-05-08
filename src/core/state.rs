use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Gen,
    Sol,
}

#[derive(Message)]
pub struct GenerationFinished;

#[derive(Message)]
pub struct PathfindingFinished;

pub fn handle_algorithm_finished(
    mut ev_gen: MessageReader<GenerationFinished>,
    mut ev_sol: MessageReader<PathfindingFinished>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if ev_gen.read().next().is_some() || ev_sol.read().next().is_some() {
        next_state.set(AppState::Idle);

        info!("Algorithm finished! Returning to Idle.");
    }
}
