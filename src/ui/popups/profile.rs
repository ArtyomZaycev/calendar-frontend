use super::popup_content::{ContentInfo, PopupContent};
use crate::app::CalendarApp;
use egui::{Align, Layout, Vec2};

pub struct Profile {}

impl Profile {
    pub fn new() -> Self {
        Self {}
    }
}

impl PopupContent for Profile {
    fn show_title(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
            if ui.small_button("X").clicked() {
                info.close();
            }
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.heading(&app.state.get_me().name);
            });
        });
        ui.separator();
    }

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, _info: &mut ContentInfo) {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Email: ");
                ui.label(&app.state.get_me().email);
            });
        });
    }
}
