use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MyWorldCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Default)]
pub struct Money {
    pub amount: u32,
}