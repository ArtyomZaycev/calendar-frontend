use super::signal::{AppSignal, RequestSignal};
use crate::db::aliases::Schedule;
use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

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
                            ui.menu_button("C", |ui| {
                                if ui.button("Edit").clicked() {
                                    self.signals.push(AppSignal::ChangeSchedule(*schedule_id));
                                    ui.close_menu();
                                }
                                if ui.button("Delete").clicked() {
                                    self.signals
                                        .push(RequestSignal::DeleteSchedule(*schedule_id).into());
                                    ui.close_menu();
                                }
                            });
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
