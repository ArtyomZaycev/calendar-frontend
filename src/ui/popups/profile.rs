use egui::{Align, Layout, RichText, Vec2};

use crate::{db::aliases::UserInfo, ui::widget_signal::AppSignal};

use super::popup_builder::PopupBuilder;

pub struct Profile {
    pub user_info: UserInfo,
    pub change_access_level: bool,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl Profile {
    pub fn new(user_info: UserInfo) -> Self {
        Self {
            user_info,
            change_access_level: false,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for Profile {
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui: &mut egui::Ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
                    if ui.small_button("X").clicked() {
                        self.closed = true;
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
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.toggle_value(&mut self.change_access_level, RichText::new("E"));
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.label("Access level: ");
                        if self.change_access_level {
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
                });
                if let Some(phone) = &self.user_info.user.phone {
                    ui.horizontal(|ui| {
                        ui.label("Phone: ");
                        ui.label(phone);
                    });
                }
            })
            .response
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
