use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

use crate::db::aliases::Event;

use super::widget_signal::{AppSignal, StateSignal};

pub struct EventCard<'a> {
    signals: &'a mut Vec<AppSignal>,
    desired_size: Vec2,
    event: &'a Event,
    access_level: i32,

    show_description: bool,
    show_date: bool,
    show_time: bool,
}

impl<'a> EventCard<'a> {
    pub fn new(
        signals: &'a mut Vec<AppSignal>,
        desired_size: Vec2,
        event: &'a Event,
        access_level: i32,
    ) -> Self {
        Self {
            signals,
            desired_size,
            event,
            access_level,
            show_description: true,
            show_date: true,
            show_time: true,
        }
    }

    pub fn hide_description(self) -> Self {
        Self {
            show_description: false,
            ..self
        }
    }

    pub fn hide_date(self) -> Self {
        Self {
            show_date: false,
            ..self
        }
    }

    pub fn hide_time(self) -> Self {
        Self {
            show_time: false,
            ..self
        }
    }
}

impl<'a> Widget for EventCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            let Event {
                id: event_id,
                name,
                description,
                start,
                end,
                plan_id,
                ..
            } = self.event;

            let is_planned = plan_id.is_some();
            let is_phantom = *event_id == -1;

            egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(
                    1.,
                    if is_planned {
                        Color32::BLUE
                    } else {
                        Color32::RED
                    },
                ))
                .inner_margin(4.)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                            if !is_phantom && self.access_level >= self.event.access_level {
                                ui.menu_button("C", |ui| {
                                    if ui.button("Edit").clicked() {
                                        self.signals.push(AppSignal::ChangeEvent(*event_id));
                                        ui.close_menu();
                                    }
                                    if ui.button("Delete").clicked() {
                                        self.signals
                                            .push(StateSignal::DeleteEvent(*event_id).into());
                                        ui.close_menu();
                                    }
                                });
                            }
                            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                                ui.add(egui::Label::new(name).wrap(true));
                            });
                        });
                        if self.show_description {
                            if let Some(description) = description {
                                ui.separator();
                                ui.label(description);
                            }
                        }
                        if self.show_date || self.show_time {
                            ui.separator();
                            if start.date() == end.date() {
                                let date = start.date();
                                let start = start.time();
                                let end = end.time();
                                if self.show_date {
                                    ui.label(date.format("%Y-%m-%d").to_string());
                                }
                                if self.show_time {
                                    ui.label(if start == end {
                                        start.format("%H:%M").to_string()
                                    } else {
                                        format!(
                                            "{} - {}",
                                            start.format("%H:%M").to_string(),
                                            end.format("%H:%M").to_string()
                                        )
                                    });
                                }
                            } else {
                                ui.add(egui::Label::new(
                                    egui::RichText::new("Unsupported date format")
                                        .color(Color32::RED)
                                        .small(),
                                ));
                            }
                        }
                        if is_phantom {
                            if let Some(plan_id) = plan_id {
                                ui.add_space(4.);
                                ui.vertical_centered(|ui| {
                                    if ui.button("Accept").clicked() {
                                        self.signals.push(
                                            StateSignal::AcceptScheduledEvent(
                                                start.date(),
                                                *plan_id,
                                            )
                                            .into(),
                                        )
                                    }
                                });
                            }
                        }
                    })
                })
                .response
        })
        .response
    }
}
