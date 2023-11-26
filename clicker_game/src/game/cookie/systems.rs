use bevy::prelude::*;

use crate::game::cookie::components::*;

pub fn spawn_cookies (
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("Clicker.png"),
        ..default()
    },
    Cookie { give_amount: 20 },
    ),);
}