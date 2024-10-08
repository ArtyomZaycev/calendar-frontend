use super::{
    popup::PopupType,
    popup_content::{ContentInfo, PopupContent},
};
use crate::{
    app::CalendarApp,
    db::request::RequestIdentifier,
    state::table_requests::{TableInsertRequest, TableUpdateRequest},
    tables::DbTable,
    ui::{access_level_picker::AccessLevelPicker, time_picker::TimePicker},
};
use calendar_lib::api::{event_templates::types::*, utils::*};
use chrono::NaiveTime;
use egui::TextEdit;
use std::hash::Hash;

pub struct EventTemplateInput {
    eid: egui::Id,
    pub orig_name: String,
    pub user_id: i32,

    pub id: Option<i32>,
    pub name: String,
    pub event_name: String,
    pub event_description: String,
    pub duration: NaiveTime,
    pub access_level: i32,

    update_request: Option<RequestIdentifier<TableUpdateRequest<EventTemplate>>>,
    insert_request: Option<RequestIdentifier<TableInsertRequest<EventTemplate>>>,
}

impl EventTemplateInput {
    pub fn new(eid: impl Hash, user_id: TableId) -> Self {
        Self {
            eid: egui::Id::new(eid),
            orig_name: String::default(),
            user_id,
            id: None,
            name: String::default(),
            event_name: String::default(),
            event_description: String::default(),
            duration: NaiveTime::from_hms_opt(0, 30, 0).unwrap(),
            access_level: -1,
            update_request: None,
            insert_request: None,
        }
    }

    pub fn change(eid: impl Hash, template: &EventTemplate) -> Self {
        let duration_minutes = template.duration.as_secs() as u32 / 60;
        Self {
            eid: egui::Id::new(eid),
            orig_name: template.name.clone(),
            user_id: template.user_id,
            id: Some(template.id),
            name: template.name.clone(),
            event_name: template.event_name.clone(),
            event_description: template.event_description.clone().unwrap_or_default(),
            duration: NaiveTime::from_hms_opt(duration_minutes / 60, duration_minutes % 60, 0)
                .unwrap(),
            access_level: template.access_level,
            update_request: None,
            insert_request: None,
        }
    }
}

impl PopupContent for EventTemplateInput {
    fn get_type(&self) -> PopupType {
        if self.id.is_some() {
            PopupType::UpdateEventTemplate
        } else {
            PopupType::NewEventTemplate
        }
    }

    fn init_frame(&mut self, app: &CalendarApp, info: &mut ContentInfo) {
        if let Some(identifier) = self.update_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                self.update_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }
        if let Some(identifier) = self.insert_request.as_ref() {
            if let Some(response_info) = app.state.get_response(&identifier) {
                self.insert_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }

        if self.access_level == -1 {
            self.access_level = app.get_selected_access_level();
        }
    }

    fn get_title(&mut self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("Change '{}' Event Template", self.orig_name))
        } else {
            Some("New Event Template".to_owned())
        }
    }

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
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
                    app.state
                        .get_user_state(self.user_id)
                        .access_levels
                        .get_table()
                        .get(),
                ));
            });

            info.error(self.name.is_empty(), "Name cannot be empty");
            info.error(self.name.len() > 200, "Name is too long");
            info.error(self.event_name.is_empty(), "Event name cannot be empty");
            info.error(self.event_name.len() > 200, "Event name is too long");
        });
    }

    fn show_buttons(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        if let Some(id) = self.id {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Update"))
                .clicked()
            {
                self.update_request = Some(
                    app.state
                        .get_user_state(self.user_id)
                        .event_templates
                        .update(UpdateEventTemplate {
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
                        }),
                );
            }
        } else {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Create"))
                .clicked()
            {
                self.insert_request = Some(
                    app.state
                        .get_user_state(self.user_id)
                        .event_templates
                        .insert(NewEventTemplate {
                            user_id: self.user_id,
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
                        }),
                );
            }
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
