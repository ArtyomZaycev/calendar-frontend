use super::popups::popup_manager::PopupManager;
use crate::{app::CalendarApp, db::aliases::EventTemplate};
use calendar_lib::api::permissions::types::TablePermissions;
use egui::{Align, Color32, Layout, Stroke, Vec2, Widget};

pub struct EventTemplateCard<'a> {
    app: &'a CalendarApp,
    desired_size: Vec2,
    event_template: &'a EventTemplate,
    permission: TablePermissions,
    access_level: i32,

    show_description: bool,
}

impl<'a> EventTemplateCard<'a> {
    pub fn new(
        app: &'a CalendarApp,
        desired_size: Vec2,
        template: &'a EventTemplate,
        access_level: i32,
        permission: TablePermissions,
    ) -> Self {
        Self {
            app,
            desired_size,
            event_template: template,
            access_level,
            permission,
            show_description: true,
        }
    }

    #[allow(dead_code)]
    pub fn hide_description(self) -> Self {
        Self {
            show_description: false,
            ..self
        }
    }
}

impl<'a> Widget for EventTemplateCard<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.allocate_ui(self.desired_size, |ui| {
            let EventTemplate {
                id: template_id,
                name,
                event_description,
                ..
            } = self.event_template;

            let response = egui::Frame::none()
                .rounding(4.)
                .stroke(Stroke::new(1., Color32::LIGHT_BLUE))
                .inner_margin(4.)
                .show(ui, |ui| {
                    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                        ui.add(egui::Label::new(name).wrap(true));
                        if self.show_description {
                            if let Some(description) = event_description {
                                ui.separator();
                                ui.label(description);
                            }
                        }
                    })
                })
                .response;

            if (self.permission.edit || self.permission.delete)
                && self.access_level >= self.event_template.access_level
            {
                response.context_menu(|ui| {
                    if self.permission.edit {
                        if ui.button("Изменить").clicked() {
                            PopupManager::get().open_update_event_template(&self.event_template);
                            ui.close_menu();
                        }
                    }
                    if self.permission.delete {
                        if ui.button("Удалить").clicked() {
                            self.app
                                .get_selected_user_state()
                                .event_templates
                                .delete(*template_id);
                            ui.close_menu();
                        }
                    }
                });
            };
        })
        .response
    }
}
