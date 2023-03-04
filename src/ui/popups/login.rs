use egui::InnerResponse;

use crate::{
    state::State,
    ui::widget_signal::{AppSignal, StateSignal},
    utils::is_valid_email,
};

use super::popup_builder::{ContentInfo, PopupBuilder};

pub struct Login {
    pub email: String,
    pub password: String,

    pub closed: bool,
    pub signals: Vec<AppSignal>,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
            closed: false,
            signals: vec![],
        }
    }
}

impl<'a> PopupBuilder<'a> for Login {
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
            show_input_field(ui, &mut self.email, "Email");
            show_input_field(ui, &mut self.password, "Password");

            ContentInfo::new()
                .error(
                    (&self.email != "admin" && !is_valid_email(&self.email))
                        .then_some("Email is not valid".to_owned()),
                )
                .error(
                    /*(!is_valid_password(&self.password))
                    .then_some("Invalid password".to_owned())
                    .or((!is_strong_enough_password(&self.password))
                        .then_some("Password is not strong enough".to_string()))*/
                    None,
                )
                .close_button("Cancel", &mut self.closed)
                .button(|ui, is_error| {
                    let response = ui.add_enabled(!is_error, egui::Button::new("Login"));
                    if response.clicked() {
                        self.signals.push(
                            StateSignal::Login(self.email.clone(), self.password.clone()).into(),
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
