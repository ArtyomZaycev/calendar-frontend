use super::popup_content::{ContentInfo, PopupContent};
use crate::{
    app::CalendarApp,
    state::{state_updater::StateUpdater, State},
    tables::DbTable,
    ui::access_level_picker::AccessLevelPicker,
};
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

    fn show_content(&mut self, app: &CalendarApp, ui: &mut egui::Ui, info: &mut ContentInfo) {
        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Email: ");
                ui.label(&app.state.get_me().email);
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.label("Access level: ");
                let mut level = app.get_selected_access_level();
                ui.add(AccessLevelPicker::new(
                    "profile_access_level_picker",
                    &mut level,
                    app.get_selected_user_state()
                        .access_levels
                        .get_table()
                        .get(),
                ));
                if app.get_selected_access_level() != level {
                    StateUpdater::get().push_executor(Box::new(move |state: &mut State| {
                        //state.change_access_level(level);
                    }));
                }
            });
        });
    }
}
