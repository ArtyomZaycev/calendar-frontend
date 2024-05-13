use super::popups::popup_manager::PopupManager;
use crate::{app::CalendarApp, db::aliases::Event};
use calendar_lib::api::permissions::types::TablePermissions;
use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

pub struct EventCard<'a> {
    app: &'a CalendarApp,
    desired_size: Vec2,
    event: &'a Event,
    permission: TablePermissions,
    access_level: i32,

    show_description: bool,
    show_date: bool,
    show_time: bool,
}

impl<'a> EventCard<'a> {
    pub fn new(
        app: &'a CalendarApp,
        desired_size: Vec2,
        event: &'a Event,
        access_level: i32,
        permission: TablePermissions,
    ) -> Self {
        Self {
            app,
            desired_size,
            event,
            access_level,
            permission,
            show_description: true,
            show_date: true,
            show_time: true,
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

            let response = egui::Frame::none()
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
                                        self.app
                                            .get_selected_user_state()
                                            .accept_scheduled_event(*plan_id, start.date());
                                    }
                                });
                            }
                        }
                    })
                })
                .response;

            if !is_phantom
                && (self.permission.edit || self.permission.delete)
                && self.access_level >= self.event.access_level
            {
                response.context_menu(|ui| {
                    if self.permission.edit {
                        if ui.button("Edit").clicked() {
                            PopupManager::get().open_update_event(&self.event);
                            ui.close_menu();
                        }
                    }
                    if self.permission.delete {
                        if ui.button("Delete").clicked() {
                            self.app.get_selected_user_state().events.delete(*event_id);
                            ui.close_menu();
                        }
                    }
                });
            }

            response
        })
        .response
    }
}
