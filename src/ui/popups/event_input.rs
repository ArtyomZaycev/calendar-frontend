use std::ops::RangeInclusive;

use calendar_lib::api::events::types::{Event, NewEvent, UpdateEvent};
use chrono::{Duration, NaiveDateTime};

use crate::ui::{
    widget_builder::AppWidgetBuilder,
    widget_signal::{AppSignal, StateSignal},
};

pub struct EventInput {
    pub max_access_level: i32,

    pub id: Option<i32>,
    pub name: String,
    pub description_enabled: bool,
    pub description: String,
    pub access_level: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl EventInput {
    pub fn new(max_access_level: i32) -> Self {
        let now = chrono::offset::Local::now().naive_local();
        Self {
            max_access_level,
            id: None,
            name: String::default(),
            description_enabled: false,
            description: String::default(),
            access_level: 0,
            start: now,
            end: now + Duration::minutes(30),
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
            start: event.start,
            end: event.end,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> AppWidgetBuilder<'a> for EventInput {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;
    type Signal = AppSignal;

    fn build(&'a mut self, _ctx: &'a egui::Context) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
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

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
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
                                        start: Some(self.start),
                                        end: Some(self.end),
                                        access_level: Some(self.access_level),
                                    },
                                )));
                        }
                    } else {
                        if ui.button("Create").clicked() {
                            self.signals
                                .push(AppSignal::StateSignal(StateSignal::InsertEvent(NewEvent {
                                    name: self.name.clone(),
                                    description: self
                                        .description_enabled
                                        .then_some(self.description.clone()),
                                    start: self.start,
                                    end: self.end,
                                    access_level: self.access_level,
                                })));
                        }
                    }
                });
            })
            .response
        })
    }

    fn signals(&'a self) -> Vec<Self::Signal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
