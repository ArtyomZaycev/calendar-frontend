// TODO: Move to lib

use chrono::prelude::*;
use eframe::egui::{Response, Ui, Widget};
use egui::Id;
use std::hash::Hash;

pub struct TimePicker<'a> {
    pub id: Id,
    pub time: &'a mut NaiveTime,
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
                .selected_text(hours.to_string())
                .show_ui(ui, |ui| {
                    (0..24).for_each(|hour| {
                        ui.selectable_value(&mut hours, hour, hour.to_string());
                    });
                });

            let mut minutes = self.time.minute();
            egui::ComboBox::from_id_source(self.id.with(self.id.short_debug_format()))
                .width(50.)
                .selected_text(minutes.to_string())
                .show_ui(ui, |ui| {
                    (0..60).for_each(|minute| {
                        ui.selectable_value(&mut minutes, minute, minute.to_string());
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
