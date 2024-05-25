use crate::utils::access_levels_human_name;
use calendar_lib::api::auth::types::AccessLevel;
use egui::{Id, Widget};
use itertools::Itertools;
use std::hash::Hash;

pub struct AccessLevelPicker<'a> {
    id: Id,
    access_level: &'a mut i32,
    access_levels: &'a [AccessLevel],
}

impl<'a> AccessLevelPicker<'a> {
    pub fn new(id: impl Hash, access_level: &'a mut i32, access_levels: &'a [AccessLevel]) -> Self {
        Self {
            id: Id::new(id),
            access_level,
            access_levels,
        }
    }
}

impl<'a> Widget for AccessLevelPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_source(self.id)
                .selected_text(access_levels_human_name(
                    self.access_levels,
                    *self.access_level,
                ))
                .show_ui(ui, |ui| {
                    self.access_levels
                        .iter()
                        .sorted_by_key(|al| -al.level)
                        .for_each(|level| {
                            ui.selectable_value(
                                self.access_level,
                                level.level,
                                access_levels_human_name(self.access_levels, level.level),
                            );
                        });
                });
        })
        .response
    }
}
