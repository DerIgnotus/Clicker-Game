use bevy::prelude::*;

#[derive(Component)]
pub struct Cookie {
    pub give_amount: f32,
    pub spawn_location_x: f32,
    pub spawn_location_y: f32,
    pub size: f32,
    pub cookie_image_name: String,
}