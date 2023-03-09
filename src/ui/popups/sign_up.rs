use egui::InnerResponse;

use crate::{
    state::State,
    ui::widget_signal::{AppSignal, StateSignal},
    utils::{is_strong_enough_password, is_valid_email, is_valid_password},
};

use super::popup_builder::{ContentInfo, PopupBuilder};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,

    pub email_taken: bool,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            name: String::default(),
            email: String::default(),
            password: String::default(),
            password2: String::default(),
            email_taken: false,
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for SignUp {
    fn content(
        &'a mut self,
        ui: &mut egui::Ui,
        _ctx: &'a egui::Context,
        _state: &'a State,
    ) -> InnerResponse<ContentInfo<'a>> {
        self.signals.clear();

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

            ContentInfo::new()
                .error(
                    (self.name.len() < 6).then_some("Name must be at least 6 symbols".to_owned()),
                )
                .error((!is_valid_email(&self.email)).then_some("Email is not valid".to_owned()))
                .error(
                    self.email_taken
                        .then_some("Account with this email is already registered".to_owned()),
                )
                .error(
                    (!is_valid_password(&self.password)).then_some("Invalid password".to_owned()),
                )
                .error(
                    (!is_strong_enough_password(&self.password))
                        .then_some("Password is not strong enough".to_string()),
                )
                .error(
                    (self.password != self.password2)
                        .then_some("Passwords must be the same".to_owned()),
                )
                .close_button("Cancel", &mut self.closed)
                .button(|ui, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Sign Up"));
                    if response.clicked() {
                        self.signals.push(
                            StateSignal::Register(
                                self.name.clone(),
                                self.email.clone(),
                                self.password.clone(),
                            )
                            .into(),
                        );
                    }
                    response
                })
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
