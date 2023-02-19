use crate::db::state::State;

pub trait AppWidgetBuilder<'a> {
    type Output;

    fn build(&'a mut self, state: &'a mut State, ctx: &'a egui::Context) -> Self::Output
    where
        Self::Output: egui::Widget + 'a;
}
