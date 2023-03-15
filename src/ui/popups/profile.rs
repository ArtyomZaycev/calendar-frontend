use egui::{Align, InnerResponse, Layout, Vec2};

use crate::{db::aliases::UserInfo, state::State};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct Profile {
    pub user_info: UserInfo,
}

impl Profile {
    pub fn new(user_info: UserInfo) -> Self {
        Self { user_info }
    }
}

impl<'a> PopupBuilder<'a> for Profile {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ContentUiInfo::new().builder(|builder| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
                    if ui.small_button("X").clicked() {
                        builder.close();
                    }
                    if ui.small_button("E").clicked() {
                        println!("Not implemented");
                        //self.signals.push(AppSignal::ChangeProfile());
                    }
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.heading(&self.user_info.user.name);
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Email: ");
                    ui.label(&self.user_info.user.email);
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.label("Access level: ");
                    if state.has_edit_rights() {
                        egui::ComboBox::from_id_source("profile_access_level_list")
                            .selected_text(self.user_info.get_access_level().name)
                            .show_ui(ui, |ui| {
                                self.user_info
                                    .access_levels
                                    .iter()
                                    .for_each(|access_level| {
                                        ui.selectable_value(
                                            &mut self.user_info.current_access_level,
                                            access_level.level,
                                            &access_level.name,
                                        );
                                    });
                            });
                    } else {
                        ui.label(self.user_info.get_access_level().name);
                    }
                });
            })
        })
    }
}
