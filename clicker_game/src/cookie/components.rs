use bevy::prelude::*;

#[derive(Component)]
struct Cookie {
    give_amount: f32,
    spawn_location: Vec2,
    size: f32,
}