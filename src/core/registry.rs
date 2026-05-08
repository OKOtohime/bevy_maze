use crate::core::prelude::*;
use crate::generation::prelude::*;
use crate::pathfinding::common::*;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;

// Store registered algorithm
#[derive(Resource, Default)]
pub struct AlgorithmRegistry {
    pub generators: Vec<&'static str>,
    pub solvers: Vec<&'static str>,
}

#[derive(Resource)]
pub struct SelectedAlgorithms {
    pub gen_algorithm: &'static str,
    pub sol_algorithm: &'static str,
}

impl Default for SelectedAlgorithms {
    fn default() -> Self {
        Self {
            gen_algorithm: "DFS",
            sol_algorithm: "BFS",
        }
    }
}

pub trait MazeAppExt {
    fn register_gen_algo<T, M, S>(
        &mut self,
        name: &'static str,
        setup_system: S,
    ) -> &mut Self
    where
        T: SteppedGenAlgorithm + Resource + FromWorld,
        S: IntoScheduleConfigs<ScheduleSystem, M>;

    fn register_sol_algo<T, M, S>(
        &mut self,
        name: &'static str,
        setup_system: S,
    ) -> &mut Self
    where
        T: SteppedSolAlgorithm + Resource + FromWorld,
        S: IntoScheduleConfigs<ScheduleSystem, M>;
}

impl MazeAppExt for App {
    fn register_gen_algo<T, M, S>(
        &mut self,
        name: &'static str,
        setup_system: S,
    ) -> &mut Self
    where
        T: SteppedGenAlgorithm + Resource + FromWorld,
        S: IntoScheduleConfigs<ScheduleSystem, M>,
    {
        let mut registry = self.world_mut().get_resource_or_insert_with(AlgorithmRegistry::default);
        registry.generators.push(name);
        self.init_resource::<T>();
        self.add_systems(OnEnter(AppState::Gen),
                         setup_system
                             .after(reset_map)
                             .run_if(move |sel: Res<SelectedAlgorithms>| sel.gen_algorithm == name)
        );
        self.add_systems(Update,
                         step_gen_algorithm::<T>
                             .run_if(in_state(AppState::Gen))
                             .run_if(is_ready_to_step)
                             .run_if(move |sel: Res<SelectedAlgorithms>| sel.gen_algorithm == name)
        );
        self
    }

    fn register_sol_algo<T, M, S>(
        &mut self,
        name: &'static str,
        setup_system: S,
    ) -> &mut Self
    where
        T: SteppedSolAlgorithm + Resource + FromWorld,
        S: IntoScheduleConfigs<ScheduleSystem, M>,
    {
        let mut registry = self.world_mut().get_resource_or_insert_with(AlgorithmRegistry::default);
        registry.solvers.push(name);
        // init state
        self.init_resource::<T>();
        // setup system
        self.add_systems(OnEnter(AppState::Sol),
                         setup_system
                             .after(clear_previous_path)
                             .run_if(move |sel: Res<SelectedAlgorithms>| sel.sol_algorithm == name)
        );
        // step system
        self.add_systems(Update,
                         step_sol_algorithm::<T>
                             .run_if(in_state(AppState::Sol))
                             .run_if(is_ready_to_step)
                             .run_if(move |sel: Res<SelectedAlgorithms>| sel.sol_algorithm == name)
        );
        self
    }
}
