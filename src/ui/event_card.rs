use egui::{Layout, Vec2, Widget};

use crate::db::aliases::Event;

pub struct EventCard<'a> {
    max_size: Vec2,
    event: &'a Event,
}

impl<'a> EventCard<'a> {
    pub fn new(event: &'a Event) -> Self {
        Self {
            max_size: Vec2::new(100., 100.),
            event,
        }
    }
}

impl<'a> Widget for EventCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.max_size, |ui| {
            let Event {
                id,
                user_id,
                name,
                description,
                start,
                end,
                access_level,
            } = self.event;
            ui.label(id.to_string());
            ui.separator();
            ui.label(name);
            if let Some(description) = description {
                ui.label(description);
            }
        })
        .response
    }
}
