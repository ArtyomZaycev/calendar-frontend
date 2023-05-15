use crate::utils::event_visibility_human_name;
use calendar_lib::api::events::types::EventVisibility;
use egui::{Id, Widget, WidgetText};
use std::hash::Hash;

pub struct EventVisibilityPicker<'a> {
    id: Id,
    visibility: &'a mut EventVisibility,
    label: Option<WidgetText>,
}

impl<'a> EventVisibilityPicker<'a> {
    pub fn new(id: impl Hash, visibility: &'a mut EventVisibility) -> Self {
        Self {
            id: Id::new(id),
            visibility,
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

impl<'a> Widget for EventVisibilityPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(label) = self.label {
                ui.label(label);
            }
            egui::ComboBox::from_id_source(self.id)
                .selected_text(event_visibility_human_name(&self.visibility))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        self.visibility,
                        EventVisibility::HideName,
                        event_visibility_human_name(&EventVisibility::HideName),
                    );
                    ui.selectable_value(
                        self.visibility,
                        EventVisibility::HideDescription,
                        event_visibility_human_name(&EventVisibility::HideDescription),
                    );
                    ui.selectable_value(
                        self.visibility,
                        EventVisibility::HideAll,
                        event_visibility_human_name(&EventVisibility::HideAll),
                    );
                });
        })
        .response
    }
}
