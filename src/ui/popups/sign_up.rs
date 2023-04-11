use egui::InnerResponse;

use crate::{
    state::State,
    ui::widget_signal::StateSignal,
    utils::{is_password_strong_enough, is_valid_email},
};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,

    email_taken: Option<String>,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            password: String::default(),
            password2: String::default(),
            email_taken: None,
        }
    }

    pub fn email_taken(&mut self) {
        self.email_taken = Some(self.email.clone());
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
        let show_input_field = |ui: &mut egui::Ui, value: &mut String, hint: &str, password: bool| {
            ui.add(
                egui::TextEdit::singleline(value)
                    .desired_width(f32::INFINITY)
                    .hint_text(hint)
                    .password(password),
            );
        };

        ui.vertical_centered(|ui| {
            show_input_field(ui, &mut self.name, "Name", false);
            show_input_field(ui, &mut self.email, "Email", false);
            show_input_field(ui, &mut self.password, "Password", true);
            show_input_field(ui, &mut self.password2, "Confirm Password", true);

            ContentUiInfo::new()
                .error(self.name.len() < 6, "Name must be at least 6 symbols")
                .error(self.name.len() > 30, "Name must be at most 30 symbols")
                .error(!is_valid_email(&self.email), "Email is not valid")
                .error(
                    self.email_taken.as_ref().map_or(false, |e| e == &self.email),
                    "Account with this email is already registered",
                )
                .error(
                    !is_password_strong_enough(&self.password),
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
