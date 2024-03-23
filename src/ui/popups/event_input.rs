use super::popup_content::PopupContent;
use crate::{
    db::request::{RequestDescription, RequestId},
    state::{
        main_state::RequestIdentifier,
        table_requests::{TableInsertRequest, TableUpdateRequest},
        State,
    },
    tables::DbTable,
    ui::{
        access_level_picker::AccessLevelPicker, event_visibility_picker::EventVisibilityPicker,
        signal::RequestSignal, time_picker::TimePicker,
    },
};
use calendar_lib::api::{events::types::*, utils::*};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use egui::TextEdit;
use egui_extras::DatePickerButton;
use std::hash::Hash;

pub struct EventInput {
    eid: egui::Id,
    pub orig_name: String,
    pub user_id: i32,

    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub access_level: i32,
    pub visibility: EventVisibility,

    pub date: NaiveDate,
    pub start: NaiveTime,
    pub end: NaiveTime,

    update_request: Option<RequestIdentifier<TableUpdateRequest<Event>>>,
    insert_request: Option<RequestIdentifier<TableInsertRequest<Event>>>,
}

impl EventInput {
    pub fn new(eid: impl Hash) -> Self {
        let now = Local::now().naive_local();
        Self {
            eid: egui::Id::new(eid),
            orig_name: String::default(),
            user_id: -1,
            id: None,
            name: String::default(),
            description: String::default(),
            access_level: -1,
            visibility: EventVisibility::HideAll,
            date: now.date(),
            start: now.time(),
            end: now.time() + Duration::minutes(30),
            update_request: None,
            insert_request: None,
        }
    }

    pub fn change(eid: impl Hash, event: &Event) -> Self {
        Self {
            eid: egui::Id::new(eid),
            orig_name: event.name.clone(),
            user_id: -1,
            id: Some(event.id),
            name: event.name.clone(),
            description: event.description.clone().unwrap_or_default(),
            access_level: event.access_level,
            visibility: event.visibility,
            date: event.start.date(),
            start: event.start.time(),
            end: event.end.time(),
            update_request: None,
            insert_request: None,
        }
    }

    /// Works only for new event
    pub fn with_user_id(self, user_id: i32) -> Self {
        Self { user_id, ..self }
    }
}

impl PopupContent for EventInput {
    fn init_frame(&mut self, state: &State, info: &mut super::popup_content::ContentInfo) {
        if let Some(identifier) = self.update_request.as_ref() {
            if let Some(response_info) = state.get_response(&identifier) {
                self.update_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }
        if let Some(identifier) = self.insert_request.as_ref() {
            if let Some(response_info) = state.get_response(&identifier) {
                self.insert_request = None;
                if !response_info.is_err() {
                    info.close();
                }
            }
        }

        if self.access_level == -1 {
            self.access_level = state.get_access_level().level;
        }
    }

    fn get_title(&mut self) -> Option<String> {
        if self.id.is_some() {
            Some(format!("Change '{}' Event", self.orig_name))
        } else {
            Some("New Event".to_owned())
        }
    }

    fn show_content(
        &mut self,
        state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.description).hint_text("Description"));

            ui.horizontal(|ui| {
                ui.label("Access level: ");
                ui.add(AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    state.user_state.access_levels.get_table().get(),
                ));
            });
            ui.add(
                EventVisibilityPicker::new(self.eid.with("visibility"), &mut self.visibility)
                    .with_label("Visibility: "),
            );

            ui.add(DatePickerButton::new(&mut self.date).show_icon(false));

            ui.horizontal(|ui| {
                ui.add(TimePicker::new(
                    self.eid.with("time_start"),
                    &mut self.start,
                ));
                ui.label("-");
                self.end = self.end.max(self.start);
                ui.add(TimePicker::new(self.eid.with("time_end"), &mut self.end));
            });

            info.error(self.name.is_empty(), "Name cannot be empty");
            info.error(self.name.len() > 80, "Name is too long");
            info.error(self.description.len() > 250, "Description is too long");
        });
    }

    fn show_buttons(
        &mut self,
        state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        if let Some(id) = self.id {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Save"))
                .clicked()
            {
                self.update_request = Some(state.user_state.events.update(UpdateEvent {
                    id,
                    name: USome(self.name.clone()),
                    description: USome(
                        (!self.description.is_empty()).then_some(self.description.clone()),
                    ),
                    start: USome(NaiveDateTime::new(self.date, self.start)),
                    end: USome(NaiveDateTime::new(self.date, self.end)),
                    access_level: USome(self.access_level),
                    visibility: USome(self.visibility),
                    plan_id: UNone,
                }));
            }
        } else {
            if ui
                .add_enabled(!info.is_error(), egui::Button::new("Create"))
                .clicked()
            {
                self.insert_request = Some(state.user_state.events.insert(NewEvent {
                    user_id: self.user_id,
                    name: self.name.clone(),
                    description: (!self.description.is_empty()).then_some(self.description.clone()),
                    start: NaiveDateTime::new(self.date, self.start),
                    end: NaiveDateTime::new(self.date, self.end),
                    access_level: self.access_level,
                    visibility: self.visibility,
                    plan_id: None,
                }));
            }
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
