use egui::{Align, Layout, Vec2};

use crate::{db::aliases::UserInfo, ui::widget_signal::AppSignal};

use super::popup_builder::PopupBuilder;

pub struct Profile {
    pub user_info: UserInfo,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl Profile {
    pub fn new(user_info: UserInfo) -> Self {
        Self {
            user_info,
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
