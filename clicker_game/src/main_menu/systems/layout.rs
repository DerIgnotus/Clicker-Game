use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu (
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok (main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
} 

pub fn build_main_menu (
    commands: &mut Commands, asset_server: &Res<AssetServer>
) -> Entity {
    let main_menut_entity = commands
        .spawn ((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: Color::Rgba { red: (0.10), green: (0.10), blue: (0.10), alpha: (0.0) }.into(),
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title

            // Play
            parent.spawn (
                (
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Play",
                                    TextStyle { 
                                        font: asset_server.load("FiraSans-Bold.ttf"), 
                                        font_size: 32.0, 
                                        color: Color::WHITE, 
                                    }
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            })
            ;
            // Quit
        })
        .id();
    
    main_menut_entity
}