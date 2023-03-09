use std::ops::RangeInclusive;

use calendar_lib::api::{events::types::*, utils::*};
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use egui::InnerResponse;

use crate::{
    state::State,
    ui::{
        date_picker::DatePicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
    utils::event_visibility_human_name,
};

use super::popup_builder::{ContentInfo, PopupBuilder};

pub struct EventInput {
    pub max_access_level: i32,

    pub id: Option<i32>,
    pub name: String,
    pub description_enabled: bool,
    pub description: String,
    pub access_level: i32,
    pub visibility: EventVisibility,

    pub date: NaiveDate,
    pub start: NaiveTime,
    pub end: NaiveTime,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl EventInput {
    pub fn new(max_access_level: i32) -> Self {
        let now = Local::now().naive_local();
        Self {
            max_access_level,
            id: None,
            name: String::default(),
            description_enabled: false,
            description: String::default(),
            access_level: 0,
            visibility: EventVisibility::HideAll,
            date: now.date(),
            start: now.time(),
            end: now.time() + Duration::minutes(30),
            closed: false,
            signals: vec![],
        }
    }

    pub fn change(max_access_level: i32, event: &Event) -> Self {
        Self {
            max_access_level,
            id: Some(event.id),
            name: event.name.clone(),
            description_enabled: event.description.is_some(),
            description: event.description.clone().unwrap_or_default(),
            access_level: event.access_level,
            visibility: event.visibility,
            date: event.start.date(),
            start: event.start.time(),
            end: event.end.time(),
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for EventInput {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        _state: &'a State,
    ) -> InnerResponse<ContentInfo<'a>> {
        self.signals.clear();

        ui.vertical(|ui| {
            ui.text_edit_singleline(&mut self.name);
            ui.checkbox(&mut self.description_enabled, "Description");

            if self.description_enabled {
                ui.text_edit_multiline(&mut self.description);
            }

            ui.add(egui::Slider::new(
                &mut self.access_level,
                RangeInclusive::new(0, self.max_access_level),
            ));
            egui::ComboBox::from_id_source(self.id)
                .selected_text(event_visibility_human_name(&self.visibility))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.visibility,
                        EventVisibility::HideName,
                        event_visibility_human_name(&EventVisibility::HideName),
                    );
                    ui.selectable_value(
                        &mut self.visibility,
                        EventVisibility::HideDescription,
                        event_visibility_human_name(&EventVisibility::HideDescription),
                    );
                    ui.selectable_value(
                        &mut self.visibility,
                        EventVisibility::HideAll,
                        event_visibility_human_name(&EventVisibility::HideAll),
                    );
                });

            ui.add(DatePicker::new("date_picker_id", &mut self.date));

            ui.add(TimePicker::new("event-builder-time-start", &mut self.start));
            ui.add(TimePicker::new("event-builder-time-end", &mut self.end));

            ContentInfo::new()
                .error(
                    self.name
                        .is_empty()
                        .then_some("Name cannot be empty".to_owned()),
                )
                .error((self.start > self.end).then_some("End must be before the start".to_owned()))
                .button(|ui, _| {
                    let response = ui.button("Cancel");
                    if response.clicked() {
                        self.closed = true;
                    }
                    response
                })
                .button(|ui, is_error| {
                    if let Some(id) = self.id {
                        let response = ui.add_enabled(!is_error, egui::Button::new("Update"));
                        if response.clicked() {
                            self.signals
                                .push(AppSignal::StateSignal(StateSignal::UpdateEvent(
                                    UpdateEvent {
                                        id,
                                        name: USome(self.name.clone()),
                                        description: USome(
                                            self.description_enabled
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
                            self.signals
                                .push(AppSignal::StateSignal(StateSignal::InsertEvent(NewEvent {
                                    user_id: -1,
                                    name: self.name.clone(),
                                    description: self
                                        .description_enabled
                                        .then_some(self.description.clone()),
                                    start: NaiveDateTime::new(self.date, self.start),
                                    end: NaiveDateTime::new(self.date, self.end),
                                    access_level: self.access_level,
                                    visibility: self.visibility,
                                    plan_id: None,
                                })));
                        }
                        response
                    }
                })
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
