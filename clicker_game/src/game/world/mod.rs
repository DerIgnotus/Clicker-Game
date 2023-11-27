use bevy::prelude::*;

pub mod resources;
pub mod components;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
        .init_resource::<Money>()
        .add_systems(Update,(
            mouse_button_input
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
            my_cursor_system
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        ));
    }
}