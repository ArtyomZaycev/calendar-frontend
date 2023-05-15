use super::popup_builder::{ContentUiInfo, PopupBuilder};
use crate::{
    state::State,
    ui::widget_signal::StateSignal,
    utils::{is_password_strong_enough, is_password_valid, is_valid_email},
};
use egui::InnerResponse;

pub struct Login {
    pub email: String,
    pub password: String,
    email_not_found: Option<String>,
    password_not_found: Option<String>,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
            email_not_found: None,
            password_not_found: None,
        }
    }

    pub fn user_not_found(&mut self) {
        self.email_not_found = Some(self.email.clone());
        self.password_not_found = Some(self.password.clone());
    }
}

impl<'a> PopupBuilder<'a> for Login {
    fn title(&self) -> Option<String> {
        Some("Login".to_owned())
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        _state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        let show_input_field =
            |ui: &mut egui::Ui, value: &mut String, hint: &str, password: bool| {
                ui.add(
                    egui::TextEdit::singleline(value)
                        .desired_width(f32::INFINITY)
                        .hint_text(hint)
                        .password(password),
                );
            };

        ui.vertical_centered(|ui| {
            show_input_field(ui, &mut self.email, "Email", false);
            show_input_field(ui, &mut self.password, "Password", true);

            ContentUiInfo::new()
                .error(
                    &self.email != "admin" && !is_valid_email(&self.email),
                    "Email is not valid",
                )
                .error(!is_password_valid(&self.password), "Password is too long")
                .error(
                    &self.email != "admin@aspid.xyz" && !is_password_strong_enough(&self.password),
                    "Password is not strong enough",
                )
                .error(
                    self.email_not_found
                        .as_ref()
                        .map_or(false, |e| e == &self.email)
                        && self
                            .password_not_found
                            .as_ref()
                            .map_or(false, |e| e == &self.password),
                    "Unknown login",
                )
                .close_button("Cancel")
                .button(|ui, builder, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Login"));
                    if response.clicked() {
                        builder.signal(StateSignal::Login(
                            self.email.clone(),
                            self.password.clone(),
                        ));
                    }
                    response
                })
        })
    }
}
