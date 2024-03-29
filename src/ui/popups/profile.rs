use super::popup_content::{ContentInfo, PopupContent};
use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        signal::{AppSignal, StateSignal},
    },
};
use egui::{Align, Layout, Vec2};

pub struct Profile {}

impl Profile {
    pub fn new() -> Self {
        Self {}
    }
}

impl PopupContent for Profile {
    fn show_title(&mut self, state: &State, ui: &mut egui::Ui, info: &mut ContentInfo) {
        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(4., 0.);
            if ui.small_button("X").clicked() {
                info.close();
            }
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.heading(&state.get_me().as_ref().unwrap().user.name);
            });
        });
        ui.separator();
    }

    fn show_content(&mut self, state: &State, ui: &mut egui::Ui, info: &mut ContentInfo) {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Email: ");
                ui.label(&state.get_me().as_ref().unwrap().user.email);
            });
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                if state
                    .get_access_levels()
                    .iter()
                    .any(|access_level| access_level.is_full())
                {
                    // TODO: Somehow prevent opening multiple
                    if ui.add_enabled(true, egui::Button::new("add")).clicked() {
                        info.signal(AppSignal::AddPassword);
                    }
                }

                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.label("Access level: ");
                    let mut level = state.get_access_level().level;
                    ui.add(AccessLevelPicker::new(
                        "profile_access_level_picker",
                        &mut level,
                        state.get_access_levels(),
                    ));
                    if state.get_access_level().level != level {
                        info.signal(StateSignal::ChangeAccessLevel(level));
                    }
                });
            });
        });
    }
}
