use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme}, 
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::DefaultInspectorConfigPlugin;

pub mod game;
pub mod main_menu;
pub mod debug_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use debug_menu::DebugUiPlugin;

use debug_menu::resources::PrintsStruct;

fn main () {
    App::new ()
        .add_plugins ((DefaultPlugins.set(WindowPlugin {
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
        }),))
        .add_plugins(EguiPlugin)
        .add_plugins(DefaultInspectorConfigPlugin)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(DebugUiPlugin)
        .add_plugins(GamePlugin)
        .add_systems(PostStartup, post_startup)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            transition_to_game_state,
            transition_to_main_menu_state,
        ))
        .run();
}



fn setup (mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn post_startup (mut print: ResMut<PrintsStruct>) {
    println!("\n\n Game Starts, Have Fun!\n");
    print.add_print("Game Starts, Have Fun!".to_string());
    println!("You are in the Main Menu and the Game is Paused!");
    print.add_print("You are in the Main Menu and the Game is Paused!".to_string());
} 

pub fn transition_to_game_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut print: ResMut<PrintsStruct>,
) {
    let a_state = app_state.get();
    if keyboard_input.just_pressed(KeyCode::G) {
        if a_state != &AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered Game!");
            print.add_print("Entered Game!".to_string());
        }
    }
}

pub fn transition_to_main_menu_state (
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut print: ResMut<PrintsStruct>,
) {
    let a_state = app_state.get();
    if keyboard_input.just_pressed(KeyCode::M) {
        if a_state != &AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Entered MainMenu!");
            print.add_print("Entered MainMenu!".to_string());
        }
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}


