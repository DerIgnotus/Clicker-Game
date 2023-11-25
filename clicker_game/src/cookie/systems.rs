use bevy::prelude::*;

use crate::cookie::components::*;


fn make_cookies () -> Cookie {
    let cookie = Cookie { give_amount: 10.0, spawn_location_x: 0.0, spawn_location_y: 0.0, size: 100.0, cookie_image_name: "Clicker.png".to_string() };
    cookie
}

pub fn spawn_cookies (
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    let cookie: Cookie = make_cookies();
    let texture = asset_server.load(cookie.cookie_image_name);

    commands.spawn((SpriteBundle {
        texture,
        transform: Transform::from_xyz(cookie.spawn_location_x, cookie.spawn_location_y, 0.0),
        ..default()
    },
    Cookie { give_amount: cookie.give_amount, spawn_location_x: cookie.spawn_location_x, spawn_location_y: cookie.spawn_location_y, size: cookie.size, cookie_image_name: "Cookie.01".to_string() },
    ),);
}