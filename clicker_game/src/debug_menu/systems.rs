use bevy::winit::WinitWindows;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::*;
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use std::io::Cursor;
use winit::window::Icon;

use crate::debug_menu::resources::*;

pub fn inspector_ui (world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    egui::SidePanel::left ("hierarchy")
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

    egui::SidePanel::right ("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Inspector");

                if ui.button("Toggle Print").clicked() {
                    let mut toggle_resource = world.get_resource_mut::<ToggleBool>().unwrap();
                    let mut toggle_print = toggle_resource.toggle_print;
                    toggle_print = !toggle_print;
                    toggle_resource.toggle_print = toggle_print;
                    println!("Toggle Print: {}", toggle_print);

                    let mut print = world.get_resource_mut::<PrintsStruct>().unwrap();
                    let format_print = format!("Toggle Print: {}", toggle_print);
                    print.add_print(format_print);
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
    egui::TopBottomPanel::bottom ("prints and inputs")
    .default_height(150.0)
    .show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Prints and Inputs");

            let print_resource = world.get_resource::<PrintsStruct>().unwrap();
            let prints_vec = &print_resource.prints;
            let prints_nums = print_resource.prints_number;

            for (index, print_text) in prints_vec.iter().enumerate().rev() {
                let print_number = prints_nums - index;// Calculate the correct print number
                ui.label(format!("print ({}): {}", print_number, print_text));
            }

            ui.allocate_space(ui.available_size());
        });
    });
}

pub fn set_window_icon (
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!("../../Assets/icon_256x256.png"));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
