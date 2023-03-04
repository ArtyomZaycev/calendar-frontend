use crate::state::State;

pub trait WidgetBuilder<'a> {
    type OutputWidget;

    fn build(&'a mut self, ctx: &'a egui::Context, state: &'a State) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a;
}
