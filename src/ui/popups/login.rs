use egui::{Align, Layout};

use crate::ui::widget_signal::{AppSignal, StateSignal};

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
        Box::new(|ui: &mut egui::Ui| {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                egui::Grid::new("login")
                    .max_col_width(ui.available_width())
                    .show(ui, |ui| {
                        ui.label("Email: ");
                        ui.text_edit_singleline(&mut self.email)
                            .on_hover_text("Email");
                        ui.end_row();

                        ui.label("Password: ");
                        ui.text_edit_singleline(&mut self.password)
                            .on_hover_text("Password");
                        ui.end_row();
                    });
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    // RTL
                    if ui.button("Login").clicked() {
                        self.signals.push(
                            StateSignal::Login(self.email.clone(), self.password.clone()).into(),
                        );
                    }
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
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
