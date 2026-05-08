use bevy::prelude::{Resource, States};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Idle,
    Gen,
    Sol,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenAlgorithm {
    #[default]
    DFS,
    Prim,
    Kruskal,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SolAlgorithm {
    #[default]
    BFS,
    Dijkstra,
    AStar,
}

#[derive(Resource, Default, PartialEq, Eq, Debug)]
pub struct AlgorithmSelection {
    pub gen_algorithm: GenAlgorithm,
    pub sol_algorithm: SolAlgorithm,
}
