use std::{fmt::Display, hash::Hash};

pub use chrono::offset::{FixedOffset, Local, Utc};
use chrono::prelude::*;
use eframe::{
    egui,
    egui::{Response, Ui, Widget},
};
use egui::Id;

pub struct TimePicker<'a> {
    id: Id,
    time: &'a mut NaiveTime,
}

impl<'a> TimePicker<'a> {
    pub fn new<T: Hash>(id: T, time: &'a mut NaiveTime) -> Self {
        Self {
            id: Id::new(id),
            time,
        }
    }
}

impl<'a> Widget for TimePicker<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let mut hours = self.time.hour();
            egui::ComboBox::from_id_source(self.id)
                .width(50.)
                .selected_text((hours + 1).to_string())
                .show_ui(ui, |ui| {
                    (0..24).for_each(|hour| {
                        ui.selectable_value(&mut hours, hour, (hour + 1).to_string());
                    });
                });

            let mut minutes = self.time.minute();
            egui::ComboBox::from_id_source(self.id.with(self.id.short_debug_format()))
                .width(50.)
                .selected_text((minutes + 1).to_string())
                .show_ui(ui, |ui| {
                    (0..60).for_each(|minute| {
                        ui.selectable_value(&mut minutes, minute, (minute + 1).to_string());
                    });
                });

            *self.time = self
                .time
                .with_hour(hours)
                .unwrap()
                .with_minute(minutes)
                .unwrap();
        })
        .response
    }
}
