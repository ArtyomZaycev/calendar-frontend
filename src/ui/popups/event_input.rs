use calendar_lib::api::{events::types::*, utils::*};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use egui::{InnerResponse, TextEdit};
use std::hash::Hash;

use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        date_picker::DatePicker,
        event_visibility_picker::EventVisibilityPicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct EventInput {
    pub eid: egui::Id,

    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub access_level: i32,
    pub visibility: EventVisibility,

    pub date: NaiveDate,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl EventInput {
    pub fn new(eid: impl Hash) -> Self {
        let now = Local::now().naive_local();
        Self {
            eid: egui::Id::new(eid),
            id: None,
            name: String::default(),
            description: String::default(),
            access_level: -1,
            visibility: EventVisibility::HideAll,
            date: now.date(),
            start: now.time(),
            end: now.time() + Duration::minutes(30),
        }
    }

    pub fn change(eid: impl Hash, event: &Event) -> Self {
        Self {
            eid: egui::Id::new(eid),
            id: Some(event.id),
            name: event.name.clone(),
            description: event.description.clone().unwrap_or_default(),
            access_level: event.access_level,
            visibility: event.visibility,
            date: event.start.date(),
            start: event.start.time(),
            end: event.end.time(),
        }
    }
}

impl<'a> PopupBuilder<'a> for EventInput {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        if self.access_level == -1 {
            self.access_level = state.me.as_ref().unwrap().current_access_level;
        }

        ui.vertical(|ui| {
            ui.add(TextEdit::singleline(&mut self.name).hint_text("Name"));
            ui.add(TextEdit::multiline(&mut self.description).hint_text("Description"));

            ui.horizontal(|ui| {
                ui.label("Access level: ");
                ui.add(AccessLevelPicker::new(
                    self.eid.with("access_level"),
                    &mut self.access_level,
                    &state.me.as_ref().unwrap().access_levels,
                ));
            });
            ui.add(
                EventVisibilityPicker::new(self.eid.with("visibility"), &mut self.visibility)
                    .with_label("Visibility: "),
            );

            ui.add(DatePicker::new("date_picker_id", &mut self.date));

            ui.horizontal(|ui| {
                ui.add(TimePicker::new("event-builder-time-start", &mut self.start));
                ui.label("-");
                ui.add(TimePicker::new("event-builder-time-end", &mut self.end));
            });

            ContentUiInfo::new()
                .error(self.name.is_empty(), "Name cannot be empty")
                .error(self.start > self.end, "End must be before the start")
                .button(|ui, builder, _| {
                    let response = ui.button("Cancel");
                    if response.clicked() {
                        builder.close();
                    }
                    response
                })
                .button(|ui, builder, is_error| {
                    if let Some(id) = self.id {
                        let response = ui.add_enabled(!is_error, egui::Button::new("Update"));
                        if response.clicked() {
                            builder.signal(AppSignal::StateSignal(StateSignal::UpdateEvent(
                                UpdateEvent {
                                    id,
                                    name: USome(self.name.clone()),
                                    description: USome(
                                        (!self.description.is_empty())
                                            .then_some(self.description.clone()),
                                    ),
                                    start: USome(NaiveDateTime::new(self.date, self.start)),
                                    end: USome(NaiveDateTime::new(self.date, self.end)),
                                    access_level: USome(self.access_level),
                                    visibility: USome(self.visibility),
                                    plan_id: UNone,
                                },
                            )));
                        }
                        response
                    } else {
                        let response = ui.add_enabled(!is_error, egui::Button::new("Create"));
                        if response.clicked() {
                            builder.signal(AppSignal::StateSignal(StateSignal::InsertEvent(
                                NewEvent {
                                    user_id: -1,
                                    name: self.name.clone(),
                                    description: (!self.description.is_empty())
                                        .then_some(self.description.clone()),
                                    start: NaiveDateTime::new(self.date, self.start),
                                    end: NaiveDateTime::new(self.date, self.end),
                                    access_level: self.access_level,
                                    visibility: self.visibility,
                                    plan_id: None,
                                },
                            )));
                        }
                        response
                    }
                })
        })
    }
}
