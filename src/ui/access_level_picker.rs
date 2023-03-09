use std::hash::Hash;

use calendar_lib::api::auth::types::AccessLevel;
use egui::{Id, Widget, WidgetText};

use crate::utils::access_levels_human_name;

pub struct AccessLevelPicker<'a> {
    id: Id,
    access_level: &'a mut i32,
    access_levels: &'a [AccessLevel],
    label: Option<WidgetText>,
}

impl<'a> AccessLevelPicker<'a> {
    pub fn new(id: impl Hash, access_level: &'a mut i32, access_levels: &'a [AccessLevel]) -> Self {
        Self {
            id: Id::new(id),
            access_level,
            access_levels,
            label: None,
        }
    }

    pub fn with_label(self, label: impl Into<WidgetText>) -> Self {
        Self {
            label: Some(label.into()),
            ..self
        }
    }
}

impl<'a> Widget for AccessLevelPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(label) = self.label {
                ui.label(label);
            }
            egui::ComboBox::from_id_source(self.id)
                .selected_text(access_levels_human_name(
                    self.access_levels,
                    *self.access_level,
                ))
                .show_ui(ui, |ui| {
                    self.access_levels.iter().for_each(|level| {
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
