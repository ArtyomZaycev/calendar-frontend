use super::popups::popup_manager::PopupManager;
use crate::{app::CalendarApp, db::aliases::Event};
use calendar_lib::api::{events::types::EventVisibility, permissions::types::TablePermissions};
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
    small: bool,
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
            small: false,
        }
    }

    pub fn small(self) -> Self {
        Self {
            small: true,
            ..self
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

impl<'a> EventCard<'a> {
    fn get_name_text(&self) -> &'a str {
        if self.event.visibility == EventVisibility::HideName
            && self.access_level < self.event.access_level
        {
            "Спрятано"
        } else {
            &self.event.name
        }
    }

    fn show_content(&self, ui: &mut egui::Ui) -> egui::Response {
        let Event {
            description,
            start,
            end,
            ..
        } = self.event;

        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.set_width(self.desired_size.x);
            ui.add(egui::Label::new(self.get_name_text()).wrap(true));
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
            /*if is_phantom {
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
            }*/
        })
        .response
    }

    fn show_content_small(&self, ui: &mut egui::Ui) -> egui::Response {
        let Event {
            id: event_id,
            start,
            plan_id,
            ..
        } = self.event;
        let is_phantom = *event_id == -1;

        let response = ui
            .with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.label(start.format("%H:%M").to_string());
                ui.add(egui::Label::new(self.get_name_text()).truncate(true));
                let pwidth = ui.available_width() - ui.style().spacing.item_spacing.x;
                if pwidth.is_sign_positive() {
                    ui.add_space(pwidth);
                }
            })
            .response;

        if is_phantom && response.double_clicked() {
            if let Some(plan_id) = plan_id {
                self.app
                    .get_selected_user_state()
                    .accept_scheduled_event(*plan_id, start.date());
            }
        }

        response
    }
}

impl<'a> Widget for EventCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            let Event {
                id: event_id,
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
                    if self.small {
                        self.show_content_small(ui);
                    } else {
                        self.show_content(ui);
                    }
                })
                .response;
            /*
                       if self.small {
                           response.context_menu(|ui| {
                               ui.add(Self {
                                   small: false,
                                   ..self
                               });
                           });
                       }
            */
            if !is_phantom
                && (self.permission.edit || self.permission.delete)
                && self.access_level >= self.event.access_level
            {
                response.context_menu(|ui| {
                    if self.permission.edit {
                        if ui.button("Изменить").clicked() {
                            PopupManager::get().open_update_event(&self.event);
                            ui.close_menu();
                        }
                    }
                    if self.permission.delete {
                        if ui.button("Удалить").clicked() {
                            self.app.get_selected_user_state().events.delete(*event_id);
                            ui.close_menu();
                        }
                    }
                });
            };
        })
        .response
    }
}
