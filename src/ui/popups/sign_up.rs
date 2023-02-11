use egui::{Align, Layout};

use crate::{db::state::State, ui::widget_builder::WidgetBuilder};

pub struct SignUp {
    pub email: String,
    pub password: String,

    pub closed: bool,
}

impl SignUp {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
            closed: false,
        }
    }
}

impl WidgetBuilder for SignUp {
    fn show(&mut self, state: &mut State, _ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
        if self.closed {
            false
        } else {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.text_edit_singleline(&mut self.email)
                    .on_hover_text("Email");
                ui.text_edit_singleline(&mut self.password)
                    .on_hover_text("Password");
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    if ui.button("Sign Up").clicked() {
                        println!("Not Implemented");
                    }
                    if ui.button("Login").clicked() {
                        state.login(&self.email, &self.password);
                    }
                })
            });
            true
        }
    }
}
