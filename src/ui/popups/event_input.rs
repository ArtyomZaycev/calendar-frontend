use std::ops::RangeInclusive;

use calendar_lib::api::events::types::*;
use chrono::{Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};
use egui::{Align, Layout};

use crate::{
    state::State,
    ui::{
        date_picker::DatePicker,
        time_picker::TimePicker,
        widget_signal::{AppSignal, StateSignal},
    },
    utils::event_visibility_human_name,
};

use super::popup_builder::PopupBuilder;

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
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
        _state: &'a State,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui| {
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

                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    // RTL

                    if let Some(id) = self.id {
                        if ui.button("Update").clicked() {
                            self.signals
                                .push(AppSignal::StateSignal(StateSignal::UpdateEvent(
                                    UpdateEvent {
                                        id,
                                        user_id: None,
                                        name: Some(self.name.clone()),
                                        description: Some(
                                            self.description_enabled
                                                .then_some(self.description.clone()),
                                        ),
                                        start: Some(NaiveDateTime::new(self.date, self.start)),
                                        end: Some(NaiveDateTime::new(self.date, self.end)),
                                        access_level: Some(self.access_level),
                                        visibility: Some(self.visibility),
                                        plan_id: None,
                                    },
                                )));
                        }
                    } else {
                        if ui.button("Create").clicked() {
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
                    }
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                });
            })
            .response
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
