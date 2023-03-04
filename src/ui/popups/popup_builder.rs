use crate::{
    state::State,
    ui::{widget_builder::WidgetBuilder, widget_signal::AppSignal},
};

pub trait PopupBuilder<'a> {
    fn build(
        &'a mut self,
        ctx: &'a egui::Context,
        state: &'a State,
    ) -> Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn title(&'a self) -> Option<String> {
        None
    }

    fn signals(&'a self) -> Vec<AppSignal>;

    fn is_closed(&'a self) -> bool;
}

impl<'a> WidgetBuilder<'a> for dyn PopupBuilder<'a> {
    type OutputWidget = Box<dyn FnOnce(&mut egui::Ui) -> egui::Response + 'a>;

    fn build(&'a mut self, ctx: &'a egui::Context, state: &'a State) -> Self::OutputWidget
    where
        Self::OutputWidget: egui::Widget + 'a,
    {
        self.build(ctx, state)
    }
}
