use crate::db::state::State;

pub trait WidgetBuilder {
    // Returns false if this widget is closed and no longer needs to be drawn
    fn show(&mut self, state: &mut State, ctx: &egui::Context, ui: &mut egui::Ui) -> bool;
}
