use super::popups::popup_manager::PopupManager;
use crate::{app::CalendarApp, db::aliases::Schedule};
use calendar_lib::api::permissions::types::TablePermissions;
use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

pub struct ScheduleCard<'a> {
    app: &'a CalendarApp,
    desired_size: Vec2,
    schedule: &'a Schedule,
    permission: TablePermissions,
    access_level: i32,
}

impl<'a> ScheduleCard<'a> {
    pub fn new(
        app: &'a CalendarApp,
        desired_size: Vec2,
        schedule: &'a Schedule,
        access_level: i32,
        permission: TablePermissions,
    ) -> Self {
        Self {
            app,
            desired_size,
            schedule,
            permission,
            access_level,
        }
    }
}

impl<'a> Widget for ScheduleCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            let Schedule {
                id: schedule_id,
                name,
                description,
                ..
            } = self.schedule;

            let response = egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(1., Color32::GREEN))
                .inner_margin(4.)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.add(egui::Label::new(name).wrap(true));
                        if let Some(description) = description {
                            ui.separator();
                            ui.label(description);
                        }
                    })
                })
                .response;

            if (self.permission.edit || self.permission.delete)
                && self.access_level >= self.schedule.access_level
            {
                response.context_menu(|ui| {
                    if self.permission.edit {
                        if ui.button("Edit").clicked() {
                            PopupManager::get().open_update_schedule(&self.schedule);
                            ui.close_menu();
                        }
                    }
                    if self.permission.delete {
                        if ui.button("Delete").clicked() {
                            self.app
                                .get_selected_user_state()
                                .schedules
                                .delete(*schedule_id);
                            ui.close_menu();
                        }
                    }
                });
            };
        })
        .response
    }
}
