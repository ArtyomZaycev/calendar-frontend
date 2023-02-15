use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::{app::CalendarApp, db::aliases::Event};

pub struct EventCard<'a> {
    app: &'a mut CalendarApp,
    max_size: Vec2,
    event: &'a Event,
}

impl<'a> EventCard<'a> {
    pub fn new(app: &'a mut CalendarApp, event: &'a Event) -> Self {
        Self {
            app,
            max_size: Vec2::new(100., 100.),
            event,
        }
    }
}

impl<'a> Widget for EventCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::none()
            .rounding(4.)
            .stroke(Stroke::new(2., Color32::RED))
            .inner_margin(4.)
            .show(ui, |ui| {
                ui.allocate_ui_with_layout(self.max_size, Layout::top_down(Align::Center), |ui| {
                    let Event {
                        id,
                        name,
                        description,
                        start,
                        end,
                        ..
                    } = self.event;
                    ui.label(name);
                    if let Some(description) = description {
                        ui.separator();
                        ui.label(description);
                    }
                    ui.separator();
                    ui.label(format!("{start} - {end}"));
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        // RTL
                        if ui.button("Change").clicked() {
                            self.app.open_change_event(self.event);
                        }
                        if ui.button("Delete").clicked() {
                            self.app.state.delete_event(*id);
                        }
                    });
                })
            })
            .response
    }
}
