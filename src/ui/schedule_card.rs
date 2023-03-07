use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::db::aliases::Schedule;

use super::widget_signal::{AppSignal, StateSignal};

pub struct ScheduleCard<'a> {
    signals: &'a mut Vec<AppSignal>,
    desired_size: Vec2,
    schedule: &'a Schedule,
}

impl<'a> ScheduleCard<'a> {
    pub fn new(
        signals: &'a mut Vec<AppSignal>,
        desired_size: Vec2,
        schedule: &'a Schedule,
    ) -> Self {
        Self {
            signals,
            desired_size,
            schedule,
        }
    }
}

impl<'a> Widget for ScheduleCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(1., Color32::GREEN))
                .inner_margin(4.)
                .show(ui, |ui| {
                    let Schedule {
                        id: schedule_id,
                        name,
                        description,
                        ..
                    } = self.schedule;
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
                            if ui.small_button("X").clicked() {
                                self.signals
                                    .push(StateSignal::DeleteSchedule(*schedule_id).into());
                            }
                            if ui.small_button("E").clicked() {
                                self.signals.push(AppSignal::ChangeSchedule(*schedule_id));
                            }
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                ui.add(egui::Label::new(name).wrap(true));
                            });
                        });
                        if let Some(description) = description {
                            ui.separator();
                            ui.label(description);
                        }
                    })
                })
                .response
        })
        .response
    }
}
