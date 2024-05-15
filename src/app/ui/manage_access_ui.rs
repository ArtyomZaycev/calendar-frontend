use egui::{Align, Button, Label, Layout, RichText, Sense};
use itertools::Itertools;

use crate::{
    app::{CalendarApp, CalendarView},
    tables::{DbTable, DbTableGetById},
    ui::popups::popup_manager::PopupManager,
};

impl CalendarApp {
    pub(super) fn manage_access_view(&mut self, ui: &mut egui::Ui, previous_view: CalendarView) {
        let permissions = self.get_selected_user_permissions();
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(Label::new(RichText::new("<").heading()).sense(Sense::click()))
                    .clicked()
                {
                    self.view = previous_view.into();
                }
                let height = ui.heading("Manage Access").rect.height();

                if permissions.allow_share {
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width(), height),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            if ui.button("Share").clicked() {
                                PopupManager::get().open_new_permission(self.selected_user_id);
                            }
                        },
                    );
                }
            });

            egui::Grid::new("access_grid").show(ui, |ui| {
                let permissions = self
                    .get_selected_user_state()
                    .granted_permissions
                    .get_table()
                    .get()
                    .iter()
                    .filter(|gp| gp.giver_user_id == self.selected_user_id)
                    .collect_vec();

                permissions.iter().for_each(|gp| {
                    let user = self
                        .get_selected_user_state()
                        .users
                        .get_table()
                        .get_by_id(gp.receiver_user_id);
                    let user_email = match user {
                        Some(user) => &user.email,
                        None => "Unknown",
                    };
                    ui.label(user_email);
                    if ui
                        .add_enabled(user.is_some(), Button::new("MANAGE"))
                        .clicked()
                    {
                        PopupManager::get().open_update_permission(&gp, user_email.to_owned());
                    }
                    if ui.button("REVOKE").clicked() {
                        self.get_selected_user_state().granted_permissions.delete(gp.id);
                    }
                })
            });
        });
    }
}
