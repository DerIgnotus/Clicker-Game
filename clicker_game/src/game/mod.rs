use bevy::prelude::*;

pub mod cookie;
pub mod world;
mod systems;

use cookie::CookiePlugin;
use world::WorldPlugin;

use crate::AppState;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins(CookiePlugin)
            .add_plugins(WorldPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}