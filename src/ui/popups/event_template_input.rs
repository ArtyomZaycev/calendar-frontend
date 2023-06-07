use super::popup_content::PopupContent;
use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        time_picker::TimePicker,
        signal::{AppSignal, StateSignal, RequestSignal},
    },
};
use calendar_lib::api::{event_templates::types::*, utils::*};
use chrono::NaiveTime;
use egui::TextEdit;
use std::hash::Hash;

pub struct EventTemplateInput {
    eid: egui::Id,
    pub orig_name: String,

    pub id: Option<i32>,
    pub name: String,
    pub event_name: String,
    pub event_description: String,
    pub duration: NaiveTime,
    pub access_level: i32,
}

impl EventTemplateInput {
    pub fn new(eid: impl Hash) -> Self {
        Self {
            eid: egui::Id::new(eid),
            orig_name: String::default(),
            id: None,
            name: String::default(),
            event_name: String::default(),
            event_description: String::default(),
            duration: NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
            access_level: -1,
        }
    }

    pub fn change(eid: impl Hash, template: &EventTemplate) -> Self {
        let duration_minutes = template.duration.as_secs() as u32 / 60;
        Self {
            eid: egui::Id::new(eid),
            orig_name: template.name.clone(),
            id: Some(template.id),
            name: template.name.clone(),
            event_name: template.event_name.clone(),
            event_description: template.event_description.clone().unwrap_or_default(),
            duration: NaiveTime::from_hms_opt(duration_minutes / 60, duration_minutes % 60, 0)
                .unwrap(),
            access_level: template.access_level,
        }
    }
}

impl PopupContent for EventTemplateInput {
    fn get_title(&mut self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("Change '{}' Event Template", self.orig_name))
        } else {
            Some("New Event Template".to_owned())
        }
    }

    fn show_content(
        &mut self,
        state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        if self.access_level == -1 {
            self.access_level = state.get_access_level().level;
        }

        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Template name"));
            ui.separator();

            ui.add(TextEdit::singleline(&mut self.event_name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.event_description).hint_text("Description"));

            ui.horizontal(|ui| {
                ui.label("Duration: ");
                ui.add(TimePicker::new(
                    "event_template_duration_picker",
                    &mut self.duration,
                ));
            });

            ui.horizontal(|ui| {
                ui.label("Access level: ");
                ui.add(AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    &state.access_levels,
                ));
            });

            info.error(self.name.is_empty(), "Name cannot be empty");
            info.error(self.name.len() > 80, "Name is too long");
            info.error(self.event_name.is_empty(), "Event name cannot be empty");
            info.error(self.event_name.len() > 80, "Event name is too long");
            info.error(
                self.event_description.len() > 250,
                "Event description is too long",
            );
        });
    }

    fn show_buttons(
        &mut self,
        _state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        if let Some(id) = self.id {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Update"))
                .clicked()
            {
                info.signal(RequestSignal::UpdateEventTemplate(
                    UpdateEventTemplate {
                        id,
                        name: USome(self.name.clone()),
                        event_name: USome(self.event_name.clone()),
                        event_description: USome(
                            (!self.event_description.is_empty())
                                .then_some(self.event_description.clone()),
                        ),
                        duration: USome(
                            self.duration
                                .signed_duration_since(NaiveTime::default())
                                .to_std()
                                .unwrap(),
                        ),
                        access_level: USome(self.access_level),
                    },
                ));
            }
        } else {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Create"))
                .clicked()
            {
                info.signal(RequestSignal::InsertEventTemplate(
                    NewEventTemplate {
                        user_id: -1,
                        name: self.name.clone(),
                        event_name: self.event_name.clone(),
                        event_description: (!self.event_description.is_empty())
                            .then_some(self.event_description.clone()),
                        duration: self
                            .duration
                            .signed_duration_since(NaiveTime::default())
                            .to_std()
                            .unwrap(),
                        access_level: self.access_level,
                    },
                ));
            }
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
