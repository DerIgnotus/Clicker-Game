use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct CookiePlugin;

impl Plugin for CookiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_cookies);
    }
}