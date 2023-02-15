use egui::{Align, Layout};

use crate::{db::state::State, ui::widget_builder::WidgetBuilder};

pub struct Login {
    pub email: String,
    pub password: String,

    pub closed: bool,
}

impl Login {
    pub fn new() -> Self {
        Self {
            email: String::default(),
            password: String::default(),
            closed: false,
        }
    }
}

impl WidgetBuilder for Login {
    fn show(&mut self, state: &mut State, _ctx: &egui::Context, ui: &mut egui::Ui) -> bool {
        if !self.closed {
            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                // TODO: Unique id
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
                    if ui.button("Login").clicked() {
                        state.login(&self.email, &self.password);
                    }
                    if ui.button("Close").clicked() {
                        self.closed = true;
                    }
                });
            });
        }
        !self.closed
    }
}
