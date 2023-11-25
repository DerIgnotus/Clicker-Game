use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct CookiePlugin;

impl Plugin for CookiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cookies);
    }
}