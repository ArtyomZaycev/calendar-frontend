use egui::InnerResponse;

use crate::{state::State, ui::widget_signal::StateSignal, utils::is_valid_email};

use super::popup_builder::{ContentUiInfo, PopupBuilder};

pub struct Login {
    pub email: String,
    pub password: String,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
        }
    }
}

impl<'a> PopupBuilder<'a> for Login {
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
            show_input_field(ui, &mut self.email, "Email");
            show_input_field(ui, &mut self.password, "Password");

            ContentUiInfo::new()
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
