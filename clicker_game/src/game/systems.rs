use bevy::prelude::*;

use crate::game::SimulationState;
use crate::debug_menu::resources::PrintsStruct;

pub fn toggle_simulation (
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
    mut print: ResMut<PrintsStruct>,
) {
    let s_state = simulation_state.get();
    if keyboard_input.just_pressed(KeyCode::Space) {
        if s_state == &SimulationState::Running {
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused!");
            print.add_print("Simulation Paused!".to_string());
        }
        if s_state == &SimulationState::Paused {
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running!");
            print.add_print("Simulation Running!".to_string());
        }
    }
} 