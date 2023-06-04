use super::popup_content::PopupContent;
use crate::{
    state::State,
    ui::{access_level_picker::AccessLevelPicker, widget_signal::StateSignal},
};
use calendar_lib::api::auth::types::NewPassword;
use egui::TextEdit;

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

impl PopupContent for NewPasswordInput {
    fn get_title(&mut self) -> Option<String> {
        Some("New Password".to_owned())
    }

    fn show_content(
        &mut self,
        state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
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

            info.error(
                !self.viewer_password_enabled && !self.editor_password_enabled,
                "At least 1 password must be set",
            );
            info.error(
                self.viewer_password.password == self.editor_password.password,
                "Passwords must be different",
            );
        });
    }

    fn show_buttons(
        &mut self,
        _state: &State,
        ui: &mut egui::Ui,
        info: &mut super::popup_content::ContentInfo,
    ) {
        if ui
            .add_enabled(!info.is_error(), egui::Button::new("Add"))
            .clicked()
        {
            info.signal(StateSignal::InsertPassword(
                self.next_password_level - 1,
                self.viewer_password_enabled
                    .then_some(self.viewer_password.clone()),
                self.editor_password_enabled
                    .then_some(self.editor_password.clone()),
            ));
        }
        if ui.button("Cancel").clicked() {
            info.close();
        }
    }
}
