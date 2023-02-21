use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::db::aliases::Event;

use super::widget_signal::{AppSignal, StateSignal};

pub struct EventCard<'a> {
    signals: &'a mut Vec<AppSignal>,
    desired_size: Vec2,
    event: &'a Event,
}

impl<'a> EventCard<'a> {
    pub fn new(signals: &'a mut Vec<AppSignal>, desired_size: Vec2, event: &'a Event) -> Self {
        Self {
            signals,
            desired_size,
            event,
        }
    }
}

impl<'a> Widget for EventCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(2., Color32::RED))
                .inner_margin(4.)
                .show(ui, |ui| {
                    let Event {
                        name,
                        description,
                        start,
                        end,
                        ..
                    } = self.event;
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
                            if ui.small_button("X").clicked() {
                                self.signals.push(StateSignal::DeleteEvent(self.event.id).into());
                            }
                            if ui.small_button("E").clicked() {
                                self.signals.push(AppSignal::ChangeEvent(self.event.id));
                            }
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                ui.add(egui::Label::new(name).wrap(true));
                            });
                        });
                        if let Some(description) = description {
                            ui.separator();
                            ui.label(description);
                        }
                        ui.separator();
                        if start.date() == end.date() {
                            let date = start.date();
                            let start = start.time();
                            let end = end.time();
                            ui.label(date.format("%Y-%m-%d").to_string());
                            ui.label(format!(
                                "{} - {}",
                                start.format("%H:%M").to_string(),
                                end.format("%H:%M").to_string()
                            ));
                        } else {
                            ui.add(egui::Label::new(
                                egui::RichText::new("Unsupported date format")
                                    .color(Color32::RED)
                                    .small(),
                            ));
                        }
                    })
                })
                .response
        })
        .response
    }
}
