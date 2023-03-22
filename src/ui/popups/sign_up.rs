use egui::InnerResponse;

use crate::{
    state::State,
    ui::widget_signal::StateSignal,
    utils::{is_strong_enough_password, is_valid_email},
};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,

    pub email_taken: bool,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            password: String::default(),
            password2: String::default(),
            email_taken: false,
        }
    }
}

impl<'a> PopupBuilder<'a> for SignUp {
    fn title(&self) -> Option<String> {
        Some("Sign Up".to_owned())
    }

    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        _state: &'a State,
    ) -> InnerResponse<ContentUiInfo<'a>> {
        let show_input_field = |ui: &mut egui::Ui, value: &mut String, hint: &str| {
            ui.add(
                egui::TextEdit::singleline(value)
                    .desired_width(f32::INFINITY)
                    .hint_text(hint),
            );
        };

        ui.vertical_centered(|ui| {
            show_input_field(ui, &mut self.name, "Name");
            show_input_field(ui, &mut self.email, "Email");
            show_input_field(ui, &mut self.password, "Password");
            show_input_field(ui, &mut self.password2, "Repeat Password");

            ContentUiInfo::new()
                .error(self.name.len() < 6, "Name must be at least 6 symbols")
                .error(!is_valid_email(&self.email), "Email is not valid")
                .error(
                    self.email_taken,
                    "Account with this email is already registered",
                )
                .error(
                    !is_strong_enough_password(&self.password),
                    "Password is not strong enough",
                )
                .error(
                    self.password != self.password2,
                    "Passwords must be the same",
                )
                .close_button("Cancel")
                .button(|ui, builder, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Sign Up"));
                    if response.clicked() {
                        builder.signal(StateSignal::Register(
                            self.name.clone(),
                            self.email.clone(),
                            self.password.clone(),
                        ));
                    }
                    response
                })
        })
    }
}
