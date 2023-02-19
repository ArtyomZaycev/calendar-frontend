use std::ops::RangeInclusive;

use calendar_lib::api::events::types::{Event, NewEvent, UpdateEvent};
use chrono::{Duration, NaiveDateTime};

use crate::{db::state::State, ui::widget_builder::AppWidgetBuilder};

pub struct EventInput {
    pub id: Option<i32>,

    pub name: String,

    pub description_enabled: bool,
    pub description: String,

    pub access_level: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,

    pub closed: bool,
}

impl EventInput {
    pub fn new() -> Self {
        let now = chrono::offset::Local::now().naive_local();
        Self {
            id: None,
            name: String::default(),
            description_enabled: false,
            description: String::default(),
            access_level: 0,
            start: now,
            end: now + Duration::minutes(30),
            closed: false,
        }
    }

    pub fn change(event: &Event) -> Self {
        Self {
            id: Some(event.id),
            name: event.name.clone(),
            description_enabled: event.description.is_some(),
            description: event.description.clone().unwrap_or_default(),
            access_level: event.access_level,
            start: event.start,
            end: event.end,
            closed: false,
        }
    }
}

impl<'a> AppWidgetBuilder<'a> for EventInput {
    type Output = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, state: &'a mut State, _ctx: &'a egui::Context) -> Self::Output
    where
        Self::Output: egui::Widget + 'a,
    {
        Box::new(|ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.checkbox(&mut self.description_enabled, "Description");

                if self.description_enabled {
                    ui.text_edit_multiline(&mut self.description);
                }

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, state.me.as_ref().unwrap().access_level),
                ));

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                    if let Some(id) = self.id {
                        if ui.button("Update").clicked() {
                            state.update_event(UpdateEvent {
                                id,
                                user_id: None,
                                name: Some(self.name.clone()),
                                description: Some(
                                    self.description_enabled.then_some(self.description.clone()),
                                ),
                                start: Some(self.start),
                                end: Some(self.end),
                                access_level: Some(self.access_level),
                            });
                        }
                    } else {
                        if ui.button("Create").clicked() {
                            state.insert_event(NewEvent {
                                name: self.name.clone(),
                                description: self
                                    .description_enabled
                                    .then_some(self.description.clone()),
                                start: self.start,
                                end: self.end,
                                access_level: self.access_level,
                            });
                        }
                    }
                });
            })
            .response
        })
    }
}
