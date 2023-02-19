use egui::{Align, Layout};

use crate::{db::state::State, ui::widget_builder::AppWidgetBuilder};

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

impl<'a> AppWidgetBuilder<'a> for Login {
    type Output = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, state: &'a mut State, _ctx: &'a egui::Context) -> Self::Output {
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
                        state.login(&self.email, &self.password);
                    }
                    if ui.button("Cancel").clicked() {
                        self.closed = true;
                    }
                });
            })
            .response
        })
    }
}
