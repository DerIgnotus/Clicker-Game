use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme, PrimaryWindow},
    input::common_conditions::input_toggle_active,
};
use bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::{
    bevy_inspector::hierarchy::SelectedEntities, DefaultInspectorConfigPlugin,
};


#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct MyWorldCoords {
    x: f32, 
    y: f32,
}

#[derive(Resource, Default)]
struct Money {
    amount: u32,
}

#[derive(Resource)]
struct ToggleBool {
    toggle_print: bool,
}


fn main() {
    App::new() 
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Clicker Game Version 1.0!".into(),
                    position: WindowPosition::At(IVec2 { x: (360), y: (140) }), //Here you can change it 
                    resolution: (1200.0, 800.0).into(), // You can also do WindowPosition::Centered(Current) or
                    present_mode: PresentMode::AutoVsync, // WindowPosition::Automatic
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(PostStartup, 
            (
            post_startup,
            )
        )
        .add_systems(Startup, 
            (
                setup,
            ) 
        )
        .add_systems(Update, 
            (
                mouse_button_input,
                my_cursor_system,
                inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
            )
        )
        .run();
}

fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    egui::SidePanel::left("hierarchy")
        .default_width(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Hierarchy");

                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                    world,
                    ui,
                    &mut selected_entities,
                );

                ui.label("Press escape to toggle UI");
                ui.allocate_space(ui.available_size());
            });
        });

    egui::SidePanel::right("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Inspector");

                let mut toggle_resource = world.get_resource_mut::<ToggleBool>().unwrap();
                let mut toggle_print = toggle_resource.toggle_print;

                if ui.button("Toggle Print").clicked() {
                    toggle_print = !toggle_print;
                    toggle_resource.toggle_print = toggle_print;
                    println!("Toggle Print: {}", toggle_print);
                }

                match selected_entities.as_slice() {
                    &[entity] => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                    }
                    entities => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                            world, entities, ui,
                        );
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        }); 
}


fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("Clicker.png");
    commands.spawn ((
        Camera2dBundle {
            ..default()
        },
        MainCamera,
    ));

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
    ));

    commands.init_resource::<MyWorldCoords>();
    commands.insert_resource::<Money>(Money { amount: 0 });
    commands.insert_resource::<ToggleBool>(ToggleBool { toggle_print: false });
}

fn post_startup() {
    println!("\n\n Game Starts, Have Fun!\n");
}

fn mouse_button_input (
    buttons: Res<Input<MouseButton>>,
    mycoords: Res<MyWorldCoords>,
    toggle_bool: Res<ToggleBool>,
    mut money: ResMut<Money>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        println!("MouseButtonWasPressed");
        if cursor_in_clicker(&*mycoords, &*toggle_bool) {
            money.amount += 10;
            println!("Money: {}", money.amount);
        }
    }
}

fn cursor_in_clicker(mycoords: &MyWorldCoords, toggle_bool: &ToggleBool) -> bool {
    if toggle_bool.toggle_print {
        eprintln!("Cursor X: {}\nCursor Y: {}", mycoords.x, mycoords.y);
    }
    (mycoords.x <= 100.0 && mycoords.x >= -100.0) &&
    (mycoords.y <= 100.0 && mycoords.y >= -100.0)
}

fn my_cursor_system (
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.x = world_position.x;
        mycoords.y = world_position.y;
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}