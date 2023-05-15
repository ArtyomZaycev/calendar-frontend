use super::popup_builder::{ContentUiInfo, PopupBuilder};
use crate::{
    state::State,
    ui::{
        access_level_picker::AccessLevelPicker,
        widget_signal::{AppSignal, StateSignal},
    },
};
use egui::{Align, InnerResponse, Layout, Vec2};

pub struct Profile {}

impl Profile {
    pub fn new() -> Self {
        Self {}
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
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.heading(&state.me.as_ref().unwrap().user.name);
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Email: ");
                    ui.label(&state.me.as_ref().unwrap().user.email);
                });
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if state
                        .access_levels
                        .iter()
                        .any(|access_level| access_level.is_full())
                    {
                        // TODO: Somehow prevent opening multiple
                        if ui.add_enabled(true, egui::Button::new("add")).clicked() {
                            builder.signal(AppSignal::AddPassword);
                        }
                    }

                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.label("Access level: ");
                        let mut level = state.get_access_level().level;
                        ui.add(AccessLevelPicker::new(
                            "profile_access_level_picker",
                            &mut level,
                            &state.access_levels,
                        ));
                        if state.get_access_level().level != level {
                            builder.signal(StateSignal::ChangeAccessLevel(level));
                        }
                    });
                });
            })
        })
    }
}
