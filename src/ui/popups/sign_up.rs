use egui::{Align, Layout, RichText, Color32};

use crate::{db::state::State, ui::widget_builder::AppWidgetBuilder, utils::{is_valid_password, is_strong_enough_password, is_valid_email}};

pub struct SignUp {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password2: String,

    pub email_taken: bool,

    pub closed: bool,
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
        }
    }
}

impl<'a> AppWidgetBuilder<'a> for SignUp {
    type Output = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, state: &'a mut State, ctx: &'a egui::Context) -> Self::Output
        where Self::Output: egui::Widget + 'a {
        Box::new(|ui| {
            let name_error = {
                (self.name.len() < 6).then_some("Name must be at least 6 symbols".to_owned())
            };
            let email_error: Option<String> = {
                (!is_valid_email(&self.email)).then_some("Email is not valid".to_owned())
                .or(self.email_taken.then_some("Account with this email is already registered".to_owned()))
            };
            let password_error: Option<String> = {
                (!is_valid_password(&self.password)).then_some("Invalid password".to_owned())
                .or((!is_strong_enough_password(&self.password)).then_some("Password is not strong enough".to_string()))
            };
            let password2_error: Option<String> = {
                (self.password != self.password2).then_some("Passwords must be the same".to_owned())
            };
    
            let error = name_error.as_ref().or(email_error.as_ref()).or(password_error.as_ref()).or(password2_error.as_ref());
    
            let show_input_field = |ui: &mut egui::Ui, value: &mut String, hint: &str| {
                ui.add(egui::TextEdit::singleline(value).desired_width(f32::INFINITY).hint_text(hint));
            };
    
            ui.vertical_centered(|ui| {
                show_input_field(ui, &mut self.name, "Name");
                show_input_field(ui, &mut self.email, "Email");
                show_input_field(ui, &mut self.password, "Password");
                show_input_field(ui, &mut self.password2, "Repeat Password");
                ui.horizontal(|ui| {
                    if let Some(error) = error {
                        ui.add(egui::Label::new(RichText::new(error).color(Color32::RED)).wrap(true));
                    }
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        // RTL
                        if ui.add_enabled(error.is_none(), egui::Button::new("Sign Up")).clicked() {
                            state.register(&self.name, &self.email, &self.password);
                        }
                        if ui.button("Cancel").clicked() {
                            self.closed = true;
                        }
                    });
                });
            }).response
        })
    }
}
