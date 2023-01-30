use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::db::{aliases::Event, state::State};

pub struct EventCard<'a> {
    state: &'a mut State,
    max_size: Vec2,
    event: &'a Event,
}

impl<'a> EventCard<'a> {
    pub fn new(state: &'a mut State, event: &'a Event) -> Self {
        Self {
            state,
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
                        if ui.button("Delete").clicked() {
                            self.state.delete_event(*id);
                        }
                    });
                })
            })
            .response
    }
}
