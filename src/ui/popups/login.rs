use egui::{Align, Layout, RichText, Color32};

use crate::{ui::widget_signal::{AppSignal, StateSignal}, utils::is_valid_email};

use super::popup_builder::PopupBuilder;

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
    fn build(
        &'a mut self,
        _ctx: &'a egui::Context,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a> {
        self.signals.clear();
        Box::new(|ui| {
            let email_error: Option<String> = {
                (&self.email != "admin" && !is_valid_email(&self.email))
                    .then_some("Email is not valid".to_owned())
            };
            let password_error: Option<String> = {
                /*(!is_valid_password(&self.password))
                    .then_some("Invalid password".to_owned())
                    .or((!is_strong_enough_password(&self.password))
                        .then_some("Password is not strong enough".to_string()))*/
                None
            };

            let error = email_error.as_ref()
                .or(password_error.as_ref());

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
                ui.horizontal(|ui| {
                    if let Some(error) = error {
                        ui.add(
                            egui::Label::new(RichText::new(error).color(Color32::RED)).wrap(true),
                        );
                    }
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        // RTL
                        if ui
                            .add_enabled(error.is_none(), egui::Button::new("Login"))
                            .clicked()
                        {
                            self.signals.push(
                                StateSignal::Login(self.email.clone(), self.password.clone()).into(),
                            );
                        }
                        if ui.button("Cancel").clicked() {
                            self.closed = true;
                        }
                    });
                });
            })
            .response
        })
    }

    fn signals(&'a self) -> Vec<AppSignal> {
        self.signals.clone()
    }

    fn is_closed(&'a self) -> bool {
        self.closed
    }
}
