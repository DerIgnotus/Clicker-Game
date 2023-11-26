use bevy::prelude::*;
use bevy::input::common_conditions::input_toggle_active;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrintsStruct>()
        .init_resource::<ToggleBool>()
        .add_systems(Startup, set_window_icon)
        .add_systems(Update, inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)));
    }
}