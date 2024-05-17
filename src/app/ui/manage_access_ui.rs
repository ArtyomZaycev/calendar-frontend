use egui::{Align, Button, Layout};
use itertools::Itertools;

use crate::{
    app::{CalendarApp, ManageAccessView},
    tables::{DbTable, DbTableGetById},
    ui::{popups::popup_manager::PopupManager, utils::UiUtils},
};

impl CalendarApp {
    pub(super) fn manage_access_view(&mut self, ui: &mut egui::Ui, view: ManageAccessView) {
        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
            let height = ui
                .horizontal(|ui| {
                    ui.selectable_header(
                        "Sharing",
                        view.is_sharing(),
                        || {
                            self.set_view(ManageAccessView::Sharing);
                        },
                    );
                    ui.selectable_header(
                        "Access Levels",
                        view.is_access_levels(),
                        || {
                            self.set_view(ManageAccessView::AccessLevels);
                        },
                    );
                })
                .response
                .rect
                .height();

            ui.allocate_ui_with_layout(
                egui::Vec2::new(ui.available_width(), height),
                Layout::right_to_left(Align::Center),
                |ui| match view {
                    ManageAccessView::Sharing => {
                        if ui
                            .add_enabled(
                                !PopupManager::get().is_open_new_permission(),
                                egui::Button::new("Share"),
                            )
                            .clicked()
                        {
                            PopupManager::get().open_new_permission(self.selected_user_id);
                        }
                    }
                    ManageAccessView::AccessLevels => {
                        
                    }
                },
            );
        });
        ui.add_space(4.);
    }

    pub(super) fn manage_access_sharing_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            egui::Grid::new("access_grid").show(ui, |ui| {
                let permissions = self
                    .get_selected_user_state()
                    .granted_permissions
                    .get_table()
                    .get()
                    .iter()
                    .filter(|gp| gp.giver_user_id == self.selected_user_id)
                    .collect_vec();

                permissions.iter().filter_map(|gp| {
                    self.get_selected_user_state().users.get_table().get_by_id(gp.receiver_user_id).map(|u| (*gp, u))
                }).for_each(|(gp, user)| {
                    ui.label(&user.name);
                    if ui
                        .add_enabled(!PopupManager::get().is_open_update_permission(), Button::new("MANAGE"))
                        .clicked()
                    {
                        PopupManager::get().open_update_permission(&gp, user);
                    }
                    // Can't revoke your own access
                    if ui.add_enabled(gp.receiver_user_id != self.state.get_me().id, Button::new("REVOKE")).clicked() {
                        self.get_selected_user_state()
                            .granted_permissions
                            .delete(gp.id);
                    }
                })
            });
        });
    }

    pub(super) fn manage_access_access_levels_view(&mut self, ui: &mut egui::Ui) {
        
    }
}
