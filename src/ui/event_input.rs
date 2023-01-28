use calendar_lib::api_types::events;
use chrono::{Duration, NaiveDateTime};
use egui::Widget;

use crate::db::state::State;

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

    pub fn make_widget<'a>(&'a mut self, state: &'a mut State) -> impl Widget + 'a {
        move |ui: &mut egui::Ui| {
            ui.vertical(|ui| {
                ui.text_edit_singleline(&mut self.name);

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
            })
            .response
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
