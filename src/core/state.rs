use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Gen,
    Sol,
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum GenAlgorithm {
    #[default]
    DFS,
    Prim,
    Kruskal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct ActiveGenState(pub GenAlgorithm);

impl ComputedStates for ActiveGenState {
    type SourceStates = (AppState, GenAlgorithm);
    // this state only exist when in Gen state
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if sources.0 == AppState::Gen {
            Some(ActiveGenState(sources.1))
        } else {
            None
        }
    }
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SolAlgorithm {
    #[default]
    BFS,
    Dijkstra,
    AStar,
    BiBFS
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ActiveSolState(pub SolAlgorithm);

impl ComputedStates for ActiveSolState {
    type SourceStates = (AppState, SolAlgorithm);
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        if sources.0 == AppState::Sol {
            Some(ActiveSolState(sources.1))
        } else {
            None
        }
    }
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
