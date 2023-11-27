use bevy::prelude::*;

use crate::game::SimulationState;

pub fn toggle_simulation (
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    let s_state = simulation_state.get();
    if keyboard_input.just_pressed(KeyCode::Space) {
        if s_state == &SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation Paused!");
        }
        if s_state == &SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation Runnung!");
        }
    }
} 