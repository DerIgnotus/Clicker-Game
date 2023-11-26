use bevy::prelude::*;
use bevy::window::PrimaryWindow;


use crate::game::world::resources::*;
use crate::debug_menu::resources::*;
use crate::game::cookie::components::*;


pub fn cursor_in_clicker (
    mycoords: &MyWorldCoords,
    toggle_bool: &ToggleBool,
    print: &mut PrintsStruct,
) -> bool {
    if toggle_bool.toggle_print {
        let format_pos = format!("Cursor X: {}! Cursor Y: {}!", mycoords.x, mycoords.y);
        print.add_print(format_pos);
    }
    (mycoords.x <= 100.0 && mycoords.x >= -100.0) && (mycoords.y <= 100.0 && mycoords.y >= -100.0)
}

pub fn my_cursor_system (
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.x = world_position.x;
        mycoords.y = world_position.y;
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}


pub fn mouse_button_input (
    buttons: Res<Input<MouseButton>>,
    mycoords: Res<MyWorldCoords>,
    toggle_bool: Res<ToggleBool>,
    mut money: ResMut<Money>,
    mut print: ResMut<PrintsStruct>,
    cookie_q: Query<&Cookie, With<Cookie>>,
) {
    let cookie = cookie_q.single();
    if buttons.just_pressed(MouseButton::Left) {
        if cursor_in_clicker(&mycoords, &toggle_bool, &mut print) {
            money.amount += cookie.give_amount;
            let money_string = format!("Money: {}", money.amount);
            print.add_print(money_string);
            println!("Money: {}", money.amount);
        }
    }
}