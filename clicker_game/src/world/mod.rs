use bevy::prelude::*;

pub mod resources;
pub mod components;
mod systems;

use resources::*;
use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyWorldCoords>()
        .init_resource::<Money>()
        .add_systems(Update, (
            mouse_button_input,
            my_cursor_system
        ));
    }
}