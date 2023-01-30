use std::ops::RangeInclusive;

use calendar_lib::api_types::events;
use chrono::{Duration, NaiveDateTime};

use crate::{db::state::State, ui::widget_builder::WidgetBuilder};

pub struct EventInput {
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
            name: String::default(),
            description_enabled: false,
            description: String::default(),
            access_level: 0,
            start: now,
            end: now + Duration::minutes(30),
            closed: false,
        }
    }
}

impl WidgetBuilder for EventInput {
    fn show(&mut self, state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
        if self.closed {
            false
        } else {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.name);
                ui.checkbox(&mut self.description_enabled, "Description");

                if self.description_enabled {
                    ui.text_edit_multiline(&mut self.description);
                }

                ui.add(egui::Slider::new(
                    &mut self.access_level,
                    RangeInclusive::new(0, state.me.as_ref().unwrap().user.access_level),
                ));

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                    if ui.button("Create").clicked() {
                        state.insert_event(&events::insert::Body {
                            name: self.name.clone(),
                            description: self
                                .description_enabled
                                .then_some(self.description.clone()),
                            start: self.start,
                            end: self.end,
                            access_level: self.access_level,
                        });
                        self.closed = true;
                    }
                });
            });
            true
        }
    }
}
