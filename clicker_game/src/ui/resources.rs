use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ToggleBool {
    pub toggle_print: bool,
}

#[derive(Resource, Default)]
pub struct PrintsStruct {
    pub prints: Vec<String>,
    pub prints_number: usize,
}

impl PrintsStruct {
    pub fn add_print(&mut self, new_print: String) {
        // Add the new print at the beginning
        self.prints.insert(0, new_print);
        self.prints_number += 1;

        // Maintain a maximum of 5 prints
        if self.prints.len() > 7 {
            self.prints.truncate(7);
        }
    }
}