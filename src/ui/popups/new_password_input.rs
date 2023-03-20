use calendar_lib::api::auth::types::NewPassword;
use egui::{InnerResponse, TextEdit};

use crate::{
    state::State,
    ui::{access_level_picker::AccessLevelPicker, widget_signal::StateSignal},
};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct NewPasswordInput {
    pub next_password_level: i32,

    pub viewer_password_enabled: bool,
    pub viewer_password: NewPassword,

    pub editor_password_enabled: bool,
    pub editor_password: NewPassword,
}

impl NewPasswordInput {
    pub fn new() -> Self {
        Self {
            next_password_level: -1,
            viewer_password_enabled: true,
            viewer_password: NewPassword {
                name: Default::default(),
                password: Default::default(),
            },
            editor_password_enabled: true,
            editor_password: NewPassword {
                name: Default::default(),
                password: Default::default(),
            },
        }
    }
}

impl<'a> PopupBuilder<'a> for NewPasswordInput {
    fn title(&self) -> Option<String> {
        Some("New Password".to_owned())
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        let show_pass_input = |ui: &mut egui::Ui,
                               enabled: &mut bool,
                               name: &mut String,
                               password: &mut String,
                               text: &str| {
            ui.vertical(|ui| {
                ui.checkbox(enabled, text);
                ui.add_enabled(*enabled, TextEdit::singleline(name).hint_text("Name"));
                ui.add_enabled(
                    *enabled,
                    TextEdit::singleline(password).hint_text("Password"),
                );
            });
        };

        ui.vertical(|ui| {
            ui.add(AccessLevelPicker::new(
                "new_password_access_level_picker",
                &mut self.next_password_level,
                &state.access_levels,
            ));

            show_pass_input(
                ui,
                &mut self.viewer_password_enabled,
                &mut self.viewer_password.name,
                &mut self.viewer_password.password,
                "Spectator",
            );
            show_pass_input(
                ui,
                &mut self.editor_password_enabled,
                &mut self.editor_password.name,
                &mut self.editor_password.password,
                "Editor",
            );

            ContentUiInfo::new()
                .error(
                    !self.viewer_password_enabled && !self.editor_password_enabled,
                    "At least 1 password must be set",
                )
                .error(
                    self.viewer_password.password == self.editor_password.password,
                    "Passwords must be different",
                )
                .close_button("Cancel")
                .button(|ui, builder, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Add"));
                    if response.clicked() {
                        builder.signal(StateSignal::InsertPassword(
                            self.next_password_level - 1,
                            self.viewer_password_enabled
                                .then_some(self.viewer_password.clone()),
                            self.editor_password_enabled
                                .then_some(self.editor_password.clone()),
                        ));
                    }
                    response
                })
        })
    }
}
